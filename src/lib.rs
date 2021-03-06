#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;
#[macro_use]
extern crate nom;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_process;

mod parsers;

use futures::prelude::*;
use futures::{Future, Stream};
use nom::types::CompleteStr;
use parsers::{engine_message, EngineMessage};
use std::io::BufReader;
use std::process::{Command, Stdio};
use tokio_core::reactor::Handle;
use tokio_io::io::{lines, write_all, Lines};
use tokio_process::{Child, ChildStdin, ChildStdout, CommandExt};

pub use parsers::{BestMove, File, Move, PromotionPiece, Rank};

pub struct Engine {
    process: Child,
    stdin: ChildStdin,
    lines: Lines<BufReader<ChildStdout>>,
}

impl Engine {
    pub fn from_path(path: String, handle: &Handle) -> impl Future<Item = Engine, Error = ()> {
        let process = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn_async(&handle)
            .expect("failed to spawn chess engine");

        Engine::new(process)
    }

    pub fn new(mut process: tokio_process::Child) -> impl Future<Item = Engine, Error = ()> {
        let lines = lines(BufReader::new(process.stdout().take().unwrap()));
        let stdin = process.stdin().take().unwrap();

        let mut engine = Engine {
            process,
            stdin,
            lines,
        };

        async_block! {
            engine = await!(engine.write("uci\n".to_string())).expect("failed to write to engine");
            engine = await!(engine.wait_for(EngineMessage::UciOk)).expect("failed to wait for uciok");

            Ok(engine)
        }
    }

    /// Waits for the engine to be ready to accept more commands.
    ///
    /// Sends the "isready" command to the engine and waits for a
    /// "readyok" response.
    #[async]
    pub fn sync(self) -> Result<Self, ()> {
        let mut engine = self;
        engine = await!(engine.write("isready\n".to_string()))?;
        println!("waiting for uciok");
        engine = await!(engine.wait_for(EngineMessage::ReadyOk))?;
        println!("got uciok");
        Ok(engine)
    }

    #[async]
    fn wait_for(self, message: EngineMessage) -> Result<Self, ()> {
        let mut engine = self;

        loop {
            let pair = await!(engine.parse_line()).expect("Couldn't parse line");
            let m = pair.0;
            engine = pair.1;

            if m == message {
                return Ok(engine);
            }
        }
    }

    /// Tells the engine that the next search will be part of a new game.
    ///
    /// This sends the "ucinewgame" command to the engine and then calls
    /// the sync() method.
    #[async]
    pub fn new_game(self) -> Result<Self, ()> {
        let engine =
            await!(self.write("ucinewgame\n".to_string())).expect("failed to write to engine");
        await!(engine.sync())
    }

    pub fn set_option(self) {
        unimplemented!();
    }

    #[async]
    pub fn set_position(self, params: String) -> Result<Self, ()> {
        await!(self.write(format!("position {}\n", params)))
    }

    #[async]
    pub fn go(self, params: String) -> Result<(Self, BestMove), ()> {
        let mut engine =
            await!(self.write(format!("go {}\n", params))).expect("failed to write to engine");

        loop {
            let pair = await!(engine.parse_line()).expect("Couldn't parse line");
            let message = pair.0;
            engine = pair.1;

            match message {
                EngineMessage::BestMove(best_move) => return Ok((engine, best_move)),
                _ => println!("{:?}", message),
            }
        }
    }

    pub fn stop(self) {
        unimplemented!();
    }

    pub fn ponder_hit(self) -> impl Future<Item = Engine, Error = ()> {
        self.write("ponderhit".to_string())
    }

    #[async]
    pub fn quit(self) -> Result<(), std::io::Error> {
        let mut self2 = self;
        self2 = await!(self2.write("quit\n".to_string())).unwrap();
        await!(self2.process.wait_with_output()).unwrap();
        Ok(())
    }

    pub fn kill(&mut self) {
        self.process.kill().expect("failed to kill engine");
    }

    /// Writes in the engine's stdin buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate async_uci;
    /// extern crate futures;
    /// extern crate tokio_core;
    ///
    /// use async_uci::Engine;
    /// use futures::future::Future;
    /// use tokio_core::reactor::Core;
    ///
    /// fn main() {
    ///     let mut core = Core::new().unwrap();
    ///     let future = Engine::from_path("stockfish".to_string(), &core.handle())
    ///         .and_then(|engine| engine.write("go nodes 1000".to_string()));
    ///     core.run(future);
    /// }
    /// ```
    #[async]
    pub fn write(self, message: String) -> Result<Self, ()> {
        print!("[gui -> engine] {}", message);

        let lines = self.lines;
        let stdin = self.stdin;
        let process = self.process;

        let res = await!(write_all(stdin, message.into_bytes()));

        match res {
            Ok((stdin, _)) => Ok(Engine {
                process,
                stdin,
                lines,
            }),
            Err(_) => panic!("failed to write to engine"),
        }
    }

    #[async]
    pub fn read_line(self) -> Result<(String, Self), ()> {
        let lines = self.lines;
        let stdin = self.stdin;
        let process = self.process;

        let res = await!(lines.into_future());

        match res {
            Ok((l, stream)) => {
                let l = l.unwrap();
                let new_self = Engine {
                    process,
                    stdin,
                    lines: stream,
                };
                println!("[engine -> gui] {}", l);

                Ok((l, new_self))
            }
            Err(_) => panic!("error reading line"),
        }
    }

    #[async]
    pub fn parse_line(self) -> Result<(EngineMessage, Self), ()> {
        let (line, self2) = await!(self.read_line()).unwrap();

        match engine_message(CompleteStr(&line)) {
            Ok((_, message)) => Ok((message, self2)),
            e => {
                println!("{:?}", e);
                panic!("error parsing line")
            } // TODO: better error management
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_core::reactor::Core;

    #[test]
    fn engine_test() {
        let mut core = Core::new().unwrap();
        let engine_future = Engine::from_path("stockfish".to_string(), &core.handle());
        let mut engine = core.run(engine_future).unwrap();
        engine = core.run(engine.go("nodes 1000".to_string())).unwrap().0;
        engine = core.run(engine.ponder_hit()).unwrap();
        engine = core.run(engine.set_position("e2e4 e7e5".to_string()))
            .unwrap();
        engine = core.run(engine.go("nodes 1000".to_string())).unwrap().0;
        core.run(engine.quit()).unwrap();
    }
}
