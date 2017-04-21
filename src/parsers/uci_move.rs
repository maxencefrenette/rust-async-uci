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

named!(pub uci_move<Move>, do_parse!(
    from: square >>
    to: square >>
    promo_piece: opt!(promotion_piece) >>
    (Move {
        from: from,
        to: to,
        promotion_piece: promo_piece
    })
));

named!(square<Square>, do_parse!(
    file: file >>
    rank: rank >>
    (Square {
        file: file,
        rank: rank
    })
));

named!(file<File>, map_opt!(
    take!(1),
    | input: &[u8] | -> Option<File> {
        match input[0] {
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

named!(rank<Rank>, map_opt!(
    take!(1),
    | input: &[u8] | -> Option<Rank> {
        match input[0] {
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

named!(promotion_piece<PromotionPiece>, map_opt!(
    take!(1),
    | input: &[u8] | -> Option<PromotionPiece> {
        match input[0] {
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
    use nom::{IResult};

    #[test]
    fn move_test() {
        assert_eq!(
            uci_move(b"e2e4 "),
            IResult::Done(
                &b" "[..],
                Move {
                    from: Square { file: File::E, rank: Rank::Second },
                    to: Square { file: File::E, rank: Rank::Fourth },
                    promotion_piece: None
                }
            )
        );

        assert_eq!(
            uci_move(b"b7b8r"),
            IResult::Done(
                &b""[..],
                Move {
                    from: Square { file: File::B, rank: Rank::Seventh },
                    to: Square { file: File::B, rank: Rank::Eight },
                    promotion_piece: Some(PromotionPiece::Rook)
                }
            )
        );
    }

    #[test]
    fn square_test() {
        assert_eq!(square(b"a1"), IResult::Done(&b""[..], Square { file: File::A, rank: Rank::First }));
        assert_eq!(square(b"c7"), IResult::Done(&b""[..], Square { file: File::C, rank: Rank::Seventh }));
        assert_eq!(square(b"e4"), IResult::Done(&b""[..], Square { file: File::E, rank: Rank::Fourth }));
        assert_eq!(square(b"h6"), IResult::Done(&b""[..], Square { file: File::H, rank: Rank::Sixth }));
    }

    #[test]
    fn file_test() {
        assert_eq!(file(b"a"), IResult::Done(&b""[..], File::A));
        assert_eq!(file(b"b"), IResult::Done(&b""[..], File::B));
        assert_eq!(file(b"c"), IResult::Done(&b""[..], File::C));
        assert_eq!(file(b"d"), IResult::Done(&b""[..], File::D));
        assert_eq!(file(b"e"), IResult::Done(&b""[..], File::E));
        assert_eq!(file(b"f"), IResult::Done(&b""[..], File::F));
        assert_eq!(file(b"g"), IResult::Done(&b""[..], File::G));
        assert_eq!(file(b"h"), IResult::Done(&b""[..], File::H));
    }

    #[test]
    fn rank_test() {
        assert_eq!(rank(b"1"), IResult::Done(&b""[..], Rank::First));
        assert_eq!(rank(b"2"), IResult::Done(&b""[..], Rank::Second));
        assert_eq!(rank(b"3"), IResult::Done(&b""[..], Rank::Third));
        assert_eq!(rank(b"4"), IResult::Done(&b""[..], Rank::Fourth));
        assert_eq!(rank(b"5"), IResult::Done(&b""[..], Rank::Fifth));
        assert_eq!(rank(b"6"), IResult::Done(&b""[..], Rank::Sixth));
        assert_eq!(rank(b"7"), IResult::Done(&b""[..], Rank::Seventh));
        assert_eq!(rank(b"8"), IResult::Done(&b""[..], Rank::Eight));
    }

    #[test]
    fn promotion_piece_test() {
        assert_eq!(promotion_piece(b"k"), IResult::Done(&b""[..], PromotionPiece::Knight));
        assert_eq!(promotion_piece(b"b"), IResult::Done(&b""[..], PromotionPiece::Bishop));
        assert_eq!(promotion_piece(b"r"), IResult::Done(&b""[..], PromotionPiece::Rook));
        assert_eq!(promotion_piece(b"q"), IResult::Done(&b""[..], PromotionPiece::Queen));
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
