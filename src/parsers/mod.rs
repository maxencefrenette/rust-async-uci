#![allow(dead_code)]

mod best_move;
mod uci_move;

use self::best_move::best_move;
use nom::{space, types::CompleteByteSlice};

pub use self::best_move::BestMove;
pub use self::uci_move::{File, Move, PromotionPiece, Rank};

#[derive(Debug, PartialEq, Eq)]
pub enum EngineMessage {
    Id,
    UciOk,
    ReadyOk,
    BestMove(BestMove),
    Info,
    UciOption,
    UnknownCommand,
}

named!(take_all<CompleteByteSlice, CompleteByteSlice>, take_while!(|_| true));

named!(id<CompleteByteSlice, EngineMessage>, do_parse!(
    tag!("id") >>
    space >>
    take_all >>
    (EngineMessage::Id)
));

named!(
    uci_ok<CompleteByteSlice, EngineMessage>,
    do_parse!(tag!("uciok") >> (EngineMessage::UciOk))
);

named!(ready_ok<CompleteByteSlice, EngineMessage>, do_parse!(
    tag!("readyok") >>
    (EngineMessage::ReadyOk)
));

named!(info<CompleteByteSlice, EngineMessage>, do_parse!(
    tag!("info") >>
    space >>
    take_all >>
    (EngineMessage::Info)
));

named!(option<CompleteByteSlice, EngineMessage>, do_parse!(
    tag!("option") >>
    space >>
    take_all >>
    (EngineMessage::UciOption)
));

named!(unknown_command<CompleteByteSlice, EngineMessage>, do_parse!(
    take_all >>
    (EngineMessage::UnknownCommand)
));

named!(pub engine_message<CompleteByteSlice, EngineMessage>, do_parse!(
    message: alt!(
        id |
        uci_ok |
        ready_ok |
        best_move |
        info |
        option |
        unknown_command
    ) >>
    (message)
));

#[cfg(test)]
mod tests {
    use super::*;
    use super::uci_move::{File, Move, Rank, Square};

    #[test]
    fn id_test() {
        assert_eq!(
            engine_message(CompleteByteSlice(b"id name Stockfish 8 64")),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::Id))
        );
        assert_eq!(
            engine_message(CompleteByteSlice(
                b"id author T. Romstad, M. Costalba, J. Kiiski, G. Linscott"
            )),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::Id))
        );
    }

    #[test]
    fn uci_ok_test() {
        assert_eq!(
            engine_message(CompleteByteSlice(b"uciok")),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::UciOk))
        );
    }

    #[test]
    fn ready_ok_test() {
        assert_eq!(
            engine_message(CompleteByteSlice(b"readyok")),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::ReadyOk))
        );
    }

    #[test]
    fn best_move_test() {
        let e2 = Square {
            file: File::E,
            rank: Rank::Second,
        };
        let e3 = Square {
            file: File::E,
            rank: Rank::Third,
        };
        let e2e3 = Move {
            from: e2,
            to: e3,
            promotion_piece: None,
        };

        assert_eq!(
            engine_message(CompleteByteSlice(b"bestmove e2e3")),
            Ok((
                CompleteByteSlice(&b""[..]),
                EngineMessage::BestMove(BestMove {
                    best_move: e2e3,
                    ponder: None,
                })
            ))
        );
    }

    #[test]
    fn info_test() {
        assert_eq!(
            engine_message(CompleteByteSlice(b"info depth 1 seldepth 1 multipv 1 score cp 90 nodes 20 nps 20000 tbhits 0 time 1 pv e2e4")), Ok((CompleteByteSlice(&b""[..]), EngineMessage::Info))
        );
        assert_eq!(
            engine_message(CompleteByteSlice(b"info depth 2 seldepth 2 multipv 1 score cp 93 nodes 47 nps 23500 tbhits 0 time 2 pv e2e4 b7b6")),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::Info))
        );
        assert_eq!(
            engine_message(CompleteByteSlice(b"info depth 3 seldepth 3 multipv 1 score cp 119 nodes 133 nps 66500 tbhits 0 time 2 pv d2d4 d7d6 e2e4")),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::Info))
        );

        assert_eq!(
            engine_message(CompleteByteSlice(
                b"info depth 29 currmove d2d4 currmovenumber 1"
            )),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::Info))
        );
        assert_eq!(
            engine_message(CompleteByteSlice(
                b"info depth 29 currmove c2c4 currmovenumber 8"
            )),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::Info))
        );
    }

    #[test]
    fn option_test() {
        assert_eq!(
            engine_message(CompleteByteSlice(
                b"option name Debug Log File type string default"
            )),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::UciOption))
        );
        assert_eq!(
            engine_message(CompleteByteSlice(
                b"option name Contempt type spin default 0 min -100 max 100"
            )),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::UciOption))
        );
        assert_eq!(
            engine_message(CompleteByteSlice(
                b"option name Threads type spin default 1 min 1 max 128"
            )),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::UciOption))
        );
        assert_eq!(
            engine_message(CompleteByteSlice(
                b"option name Hash type spin default 16 min 1 max 1048576"
            )),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::UciOption))
        );
    }

    #[test]
    fn unknown_command_test() {
        assert_eq!(
            engine_message(CompleteByteSlice(
                b"Stockfish 8 64 by T. Romstad, M. Costalba, J. Kiiski, G. Linscott"
            )),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::UnknownCommand))
        );
        assert_eq!(
            engine_message(CompleteByteSlice(b"")),
            Ok((CompleteByteSlice(&b""[..]), EngineMessage::UnknownCommand))
        );
    }
}
