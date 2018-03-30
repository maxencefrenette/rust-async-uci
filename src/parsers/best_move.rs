use super::EngineMessage;
use super::uci_move::{uci_move, Move};
use nom::{space, types::CompleteByteSlice};

#[derive(Debug, PartialEq, Eq)]
pub struct BestMove {
    pub best_move: Move,
    pub ponder: Option<Move>,
}

named!(ponder<CompleteByteSlice, Move>, do_parse!(
    space >>
    tag!("ponder") >>
    space >>
    ponder_move: uci_move >>
    (ponder_move)
));

named!(pub best_move<CompleteByteSlice, EngineMessage>, do_parse!(
    tag!("bestmove") >>
    space >>
    best_move: uci_move >>
    ponder: opt!(ponder) >>
    (EngineMessage::BestMove(BestMove {
        best_move: best_move,
        ponder: ponder
    }))
));

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::uci_move::{File, Rank, Square};

    #[test]
    fn best_move_test() {
        let g7 = Square {
            file: File::G,
            rank: Rank::Seventh,
        };
        let g1 = Square {
            file: File::G,
            rank: Rank::First,
        };
        let g7g1 = Move {
            from: g7,
            to: g1,
            promotion_piece: None,
        };

        let a1 = Square {
            file: File::A,
            rank: Rank::First,
        };
        let a7 = Square {
            file: File::A,
            rank: Rank::Seventh,
        };
        let a1a7 = Move {
            from: a1,
            to: a7,
            promotion_piece: None,
        };

        assert_eq!(
            best_move(CompleteByteSlice(b"bestmove g7g1 ponder a1a7")),
            Ok((
                CompleteByteSlice(&b""[..]),
                EngineMessage::BestMove(BestMove {
                    best_move: g7g1,
                    ponder: Some(a1a7),
                })
            ))
        );
    }

    #[test]
    fn no_ponder_test() {
        let f1 = Square {
            file: File::F,
            rank: Rank::First,
        };
        let h3 = Square {
            file: File::H,
            rank: Rank::Third,
        };
        let f1h3 = Move {
            from: f1,
            to: h3,
            promotion_piece: None,
        };

        assert_eq!(
            best_move(CompleteByteSlice(b"bestmove f1h3")),
            Ok((
                CompleteByteSlice(&b""[..]),
                EngineMessage::BestMove(BestMove {
                    best_move: f1h3,
                    ponder: None,
                })
            ))
        );
    }
}
