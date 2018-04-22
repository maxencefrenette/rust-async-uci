#![allow(dead_code)]

mod best_move;
mod uci_move;

use self::best_move::best_move;
use nom::{space, types::CompleteStr};

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

named!(take_all<CompleteStr, CompleteStr>, take_while!(|_| true));

named!(id<CompleteStr, EngineMessage>, do_parse!(
    tag!("id") >>
    space >>
    take_all >>
    (EngineMessage::Id)
));

named!(
    uci_ok<CompleteStr, EngineMessage>,
    do_parse!(tag!("uciok") >> (EngineMessage::UciOk))
);

named!(ready_ok<CompleteStr, EngineMessage>, do_parse!(
    tag!("readyok") >>
    (EngineMessage::ReadyOk)
));

named!(info<CompleteStr, EngineMessage>, do_parse!(
    tag!("info") >>
    space >>
    take_all >>
    (EngineMessage::Info)
));

named!(option<CompleteStr, EngineMessage>, do_parse!(
    tag!("option") >>
    space >>
    take_all >>
    (EngineMessage::UciOption)
));

named!(unknown_command<CompleteStr, EngineMessage>, do_parse!(
    take_all >>
    (EngineMessage::UnknownCommand)
));

named!(pub engine_message<CompleteStr, EngineMessage>, do_parse!(
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
            engine_message(CompleteStr("id name Stockfish 8 64")),
            Ok((CompleteStr(""), EngineMessage::Id))
        );
        assert_eq!(
            engine_message(CompleteStr(
                "id author T. Romstad, M. Costalba, J. Kiiski, G. Linscott"
            )),
            Ok((CompleteStr(""), EngineMessage::Id))
        );
    }

    #[test]
    fn uci_ok_test() {
        assert_eq!(
            engine_message(CompleteStr("uciok")),
            Ok((CompleteStr(""), EngineMessage::UciOk))
        );
    }

    #[test]
    fn ready_ok_test() {
        assert_eq!(
            engine_message(CompleteStr("readyok")),
            Ok((CompleteStr(""), EngineMessage::ReadyOk))
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
            engine_message(CompleteStr("bestmove e2e3")),
            Ok((
                CompleteStr(""),
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
            engine_message(CompleteStr("info depth 1 seldepth 1 multipv 1 score cp 90 nodes 20 nps 20000 tbhits 0 time 1 pv e2e4")), Ok((CompleteStr(""), EngineMessage::Info))
        );
        assert_eq!(
            engine_message(CompleteStr("info depth 2 seldepth 2 multipv 1 score cp 93 nodes 47 nps 23500 tbhits 0 time 2 pv e2e4 b7b6")),
            Ok((CompleteStr(""), EngineMessage::Info))
        );
        assert_eq!(
            engine_message(CompleteStr("info depth 3 seldepth 3 multipv 1 score cp 119 nodes 133 nps 66500 tbhits 0 time 2 pv d2d4 d7d6 e2e4")),
            Ok((CompleteStr(""), EngineMessage::Info))
        );

        assert_eq!(
            engine_message(CompleteStr("info depth 29 currmove d2d4 currmovenumber 1")),
            Ok((CompleteStr(""), EngineMessage::Info))
        );
        assert_eq!(
            engine_message(CompleteStr("info depth 29 currmove c2c4 currmovenumber 8")),
            Ok((CompleteStr(""), EngineMessage::Info))
        );
    }

    #[test]
    fn option_test() {
        assert_eq!(
            engine_message(CompleteStr(
                "option name Debug Log File type string default"
            )),
            Ok((CompleteStr(""), EngineMessage::UciOption))
        );
        assert_eq!(
            engine_message(CompleteStr(
                "option name Contempt type spin default 0 min -100 max 100"
            )),
            Ok((CompleteStr(""), EngineMessage::UciOption))
        );
        assert_eq!(
            engine_message(CompleteStr(
                "option name Threads type spin default 1 min 1 max 128"
            )),
            Ok((CompleteStr(""), EngineMessage::UciOption))
        );
        assert_eq!(
            engine_message(CompleteStr(
                "option name Hash type spin default 16 min 1 max 1048576"
            )),
            Ok((CompleteStr(""), EngineMessage::UciOption))
        );
    }

    #[test]
    fn unknown_command_test() {
        assert_eq!(
            engine_message(CompleteStr(
                "Stockfish 8 64 by T. Romstad, M. Costalba, J. Kiiski, G. Linscott"
            )),
            Ok((CompleteStr(""), EngineMessage::UnknownCommand))
        );
        assert_eq!(
            engine_message(CompleteStr("")),
            Ok((CompleteStr(""), EngineMessage::UnknownCommand))
        );
    }
}
