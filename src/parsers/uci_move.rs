use nom::{AsBytes, types::CompleteByteSlice};
use std::fmt;

#[derive(Debug,PartialEq,Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion_piece: Option<PromotionPiece>
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref p) = self.promotion_piece {
            write!(f, "{}{}{}", self.from, self.to, p)
        } else {
            write!(f, "{}{}", self.from, self.to)
        }
    }
}

#[derive(Debug,PartialEq,Eq)]
pub struct Square {
    pub file: File,
    pub rank: Rank
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h'
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eight
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            Rank::First => '1',
            Rank::Second => '2',
            Rank::Third => '3',
            Rank::Fourth => '4',
            Rank::Fifth => '5',
            Rank::Sixth => '6',
            Rank::Seventh => '7',
            Rank::Eight => '8'
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    Queen
}

impl fmt::Display for PromotionPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            PromotionPiece::Knight => 'k',
            PromotionPiece::Bishop => 'b',
            PromotionPiece::Rook => 'r',
            PromotionPiece::Queen => 'q'
        };
        write!(f, "{}", c)
    }
}

named!(pub uci_move<CompleteByteSlice, Move>, do_parse!(
    from: square >>
    to: square >>
    promo_piece: opt!(promotion_piece) >>
    (Move {
        from: from,
        to: to,
        promotion_piece: promo_piece
    })
));

named!(square<CompleteByteSlice, Square>, do_parse!(
    file: file >>
    rank: rank >>
    (Square {
        file: file,
        rank: rank
    })
));

named!(file<CompleteByteSlice, File>, map_opt!(
    take!(1),
    | input: CompleteByteSlice | -> Option<File> {
        match input.as_bytes()[0] {
            b'a' => Some(File::A),
            b'b' => Some(File::B),
            b'c' => Some(File::C),
            b'd' => Some(File::D),
            b'e' => Some(File::E),
            b'f' => Some(File::F),
            b'g' => Some(File::G),
            b'h' => Some(File::H),
            _ => None
        }
    }
));

named!(rank<CompleteByteSlice, Rank>, map_opt!(
    take!(1),
    | input: CompleteByteSlice | -> Option<Rank> {
        match input.as_bytes()[0] {
            b'1' => Some(Rank::First),
            b'2' => Some(Rank::Second),
            b'3' => Some(Rank::Third),
            b'4' => Some(Rank::Fourth),
            b'5' => Some(Rank::Fifth),
            b'6' => Some(Rank::Sixth),
            b'7' => Some(Rank::Seventh),
            b'8' => Some(Rank::Eight),
            _ => None
        }
    }
));

named!(promotion_piece<CompleteByteSlice, PromotionPiece>, map_opt!(
    take!(1),
    | input: CompleteByteSlice | -> Option<PromotionPiece> {
        match input.as_bytes()[0] {
            b'k' => Some(PromotionPiece::Knight),
            b'b' => Some(PromotionPiece::Bishop),
            b'r' => Some(PromotionPiece::Rook),
            b'q' => Some(PromotionPiece::Queen),
            _ => None
        }
    }
));

#[cfg(test)]
mod tests {
    use super::*;
    const EMPTY_SLICE: CompleteByteSlice = CompleteByteSlice(b"");

    #[test]
    fn move_test() {
        assert_eq!(
            uci_move(CompleteByteSlice(b"e2e4")),
            Ok((
                EMPTY_SLICE,
                Move {
                    from: Square { file: File::E, rank: Rank::Second },
                    to: Square { file: File::E, rank: Rank::Fourth },
                    promotion_piece: None
                }
            ))
        );

        assert_eq!(
            uci_move(CompleteByteSlice(b"b7b8r")),
            Ok((
                EMPTY_SLICE,
                Move {
                    from: Square { file: File::B, rank: Rank::Seventh },
                    to: Square { file: File::B, rank: Rank::Eight },
                    promotion_piece: Some(PromotionPiece::Rook)
                }
            ))
        );
    }

    #[test]
    fn square_test() {
        assert_eq!(
            square(CompleteByteSlice(b"a1")),
            Ok((EMPTY_SLICE, Square { file: File::A, rank: Rank::First }))
        );
        assert_eq!(
            square(CompleteByteSlice(b"c7")), Ok((EMPTY_SLICE, Square { file: File::C, rank: Rank::Seventh }))
        );
        assert_eq!(square(CompleteByteSlice(b"e4")), Ok((EMPTY_SLICE, Square { file: File::E, rank: Rank::Fourth })));
        assert_eq!(square(CompleteByteSlice(b"h6")), Ok((EMPTY_SLICE, Square { file: File::H, rank: Rank::Sixth })));
    }

    #[test]
    fn file_test() {
        assert_eq!(file(CompleteByteSlice(b"a")), Ok((EMPTY_SLICE, File::A)));
        assert_eq!(file(CompleteByteSlice(b"b")), Ok((EMPTY_SLICE, File::B)));
        assert_eq!(file(CompleteByteSlice(b"c")), Ok((EMPTY_SLICE, File::C)));
        assert_eq!(file(CompleteByteSlice(b"d")), Ok((EMPTY_SLICE, File::D)));
        assert_eq!(file(CompleteByteSlice(b"e")), Ok((EMPTY_SLICE, File::E)));
        assert_eq!(file(CompleteByteSlice(b"f")), Ok((EMPTY_SLICE, File::F)));
        assert_eq!(file(CompleteByteSlice(b"g")), Ok((EMPTY_SLICE, File::G)));
        assert_eq!(file(CompleteByteSlice(b"h")), Ok((EMPTY_SLICE, File::H)));
    }

    #[test]
    fn rank_test() {
        assert_eq!(rank(CompleteByteSlice(b"1")), Ok((EMPTY_SLICE, Rank::First)));
        assert_eq!(rank(CompleteByteSlice(b"2")), Ok((EMPTY_SLICE, Rank::Second)));
        assert_eq!(rank(CompleteByteSlice(b"3")), Ok((EMPTY_SLICE, Rank::Third)));
        assert_eq!(rank(CompleteByteSlice(b"4")), Ok((EMPTY_SLICE, Rank::Fourth)));
        assert_eq!(rank(CompleteByteSlice(b"5")), Ok((EMPTY_SLICE, Rank::Fifth)));
        assert_eq!(rank(CompleteByteSlice(b"6")), Ok((EMPTY_SLICE, Rank::Sixth)));
        assert_eq!(rank(CompleteByteSlice(b"7")), Ok((EMPTY_SLICE, Rank::Seventh)));
        assert_eq!(rank(CompleteByteSlice(b"8")), Ok((EMPTY_SLICE, Rank::Eight)));
    }

    #[test]
    fn promotion_piece_test() {
        assert_eq!(promotion_piece(CompleteByteSlice(b"k")), Ok((EMPTY_SLICE, PromotionPiece::Knight)));
        assert_eq!(promotion_piece(CompleteByteSlice(b"b")), Ok((EMPTY_SLICE, PromotionPiece::Bishop)));
        assert_eq!(promotion_piece(CompleteByteSlice(b"r")), Ok((EMPTY_SLICE, PromotionPiece::Rook)));
        assert_eq!(promotion_piece(CompleteByteSlice(b"q")), Ok((EMPTY_SLICE, PromotionPiece::Queen)));
    }

    #[test]
    fn display_test() {
        let g6 = Square { file: File::G, rank: Rank::Sixth };
        let e4 = Square { file: File::E, rank: Rank::Fourth };
        let g6e4 = Move { from: g6, to: e4, promotion_piece: None };

        assert_eq!(format!("{}", g6e4), "g6e4");

        let a2 = Square { file: File::A, rank: Rank::Second };
        let a1 = Square { file: File::A, rank: Rank::First };
        let r = PromotionPiece::Rook;
        let a2a1r = Move { from: a2, to: a1, promotion_piece: Some(r) };

        assert_eq!(format!("{}", a2a1r), "a2a1r");
    }
}
