#[macro_use] extern crate nom;

mod parsers;

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdout, Command, Stdio};
use parsers::{engine_message, EngineMessage};

pub use parsers::{BestMove, File, Move, PromotionPiece, Rank};

pub struct Engine {
    process: Child,
    buffered_stdout: BufReader<ChildStdout>
}

impl Engine {
    pub fn new(path: &str) -> Engine {
        let mut process = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn chess engine");
        
        let buf_stdout = BufReader::new(process.stdout.expect("failed to unwrap stdout"));
        process.stdout = None;

        let mut engine = Engine {
            process: process,
            buffered_stdout: buf_stdout
        };

        engine.write("uci\n");
        loop {
            if engine.parse_line() == EngineMessage::UciOk {
                break;
            }
        }

        engine
    }

    /// Waits for the engine to be ready to accept more commands.
    ///
    /// Sends the "isready" command to the engine and waits for a
    /// "readyok" response.
    pub fn sync(&mut self) {
        self.write("isready\n");
        loop {
            if self.parse_line() == EngineMessage::ReadyOk {
                break;
            }
        }
    }

    /// Tells the engine that the next search will be part of a new game.
    ///
    /// This sends the "ucinewgame" command to the engine and then calls
    /// the sync() method.
    pub fn new_game(&mut self) {
        self.write("ucinewgame\n");
        self.sync();
    }

    pub fn set_option(&mut self) {
        unimplemented!();
    }

    pub fn set_position(&mut self, params: &str) {
        self.write(format!("position {}\n", params).as_str());
    }

    pub fn go(&mut self, params: &str) -> BestMove {
        self.write(format!("go {}\n", params).as_str());
        loop {
            match self.parse_line() {
                EngineMessage::BestMove(best_move) => return best_move,
                _ => {}
            }
        }
    }

    pub fn stop(&mut self) {
        self.write("stop");
        self.process.wait()
            .expect("failed to close engine");
    }

    pub fn ponder_hit(&mut self) {
        self.write("ponderhit");
    }

    pub fn quit(&mut self) {
        self.write("quit\n");
        self.process
            .wait()
            .expect("failed to wait for process to end");
    }

    pub fn kill(&mut self) {
        self.process.kill()
            .expect("failed to kill engine");
    }

    /// Writes in the engine's stdin buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use async_uci::Engine;
    /// 
    /// let mut engine = Engine::new("stockfish");
    /// engine.write("go nodes 1000");
    /// ```
    pub fn write(&mut self, message: &str) {
        print!("[gui -> engine] {}", message);

        self.process.stdin.as_mut()
            .expect("failed to unwrap the stdin handle")
            .write_all(message.as_bytes())
            .expect("failed to write to the engine's stdin");
    }

    pub fn read_line(&mut self) -> String {
        let mut string = String::new();

        self.buffered_stdout
            .read_line(&mut string)
            .expect("failed to read from the engine's stdout");
        
        print!("[engine -> gui] {}", string);

        string
    }

    pub fn parse_line(&mut self) -> EngineMessage {
        match engine_message(self.read_line().as_bytes()) {
            Ok((_, message)) => message,
            _ => panic!("couldn't parse line from engine")
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.quit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_test() {
        let mut engine = Engine::new("stockfish");
        engine.go("nodes 1000");
        engine.ponder_hit();
        engine.set_position("e2e4 e7e5");
        engine.go("nodes 1000");
    }
}
