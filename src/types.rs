use std::fmt;
use std::ascii::AsciiExt;
use std::char;
use std::ops;
use std::str::FromStr;

use square::Square;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn fold<T>(self, white: T, black: T) -> T {
        match self {
            Color::Black => black,
            Color::White => white
        }
    }

    pub fn pawn(self)   -> Piece { Role::Pawn.of(self) }
    pub fn knight(self) -> Piece { Role::Knight.of(self) }
    pub fn bishop(self) -> Piece { Role::Bishop.of(self) }
    pub fn rook(self)   -> Piece { Role::Rook.of(self) }
    pub fn queen(self)  -> Piece { Role::Queen.of(self) }
    pub fn king(self)   -> Piece { Role::King.of(self) }
}

impl ops::Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        self.fold(Color::Black, Color::White)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Role {
    pub fn from_char(ch: char) -> Option<Role> {
        match ch {
            'p' => Some(Role::Pawn),
            'n' => Some(Role::Knight),
            'b' => Some(Role::Bishop),
            'r' => Some(Role::Rook),
            'q' => Some(Role::Queen),
            'k' => Some(Role::King),
            _ => None
        }
    }

    pub fn of(self, color: Color) -> Piece {
        Piece { color, role: self }
    }

    pub fn char(self) -> char {
        match self {
            Role::Pawn =>   'p',
            Role::Knight => 'n',
            Role::Bishop => 'b',
            Role::Rook =>   'r',
            Role::Queen =>  'q',
            Role::King =>   'k'
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

impl Piece {
    pub fn char(self) -> char {
        self.color.fold(self.role.char().to_ascii_uppercase(), self.role.char())
    }

    pub fn from_char(ch: char) -> Option<Piece> {
        if ch == ch.to_ascii_lowercase() {
            Role::from_char(ch).map(|role| role.of(Color::Black))
        } else {
            Role::from_char(ch.to_ascii_lowercase()).map(|role| role.of(Color::White))
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Normal { from: Square, to: Square, promotion: Option<Role> },
    Put { to: Square, role: Role },
    Null
}

impl Move {
    pub fn from_uci(uci: &str) -> Option<Move> {
        if uci.len() < 4 || uci.len() > 5 {
            return None
        }

        match (Square::from_str(&uci[0..2]), Square::from_str(&uci[2..4]), uci.chars().nth(4)) {
            (Ok(from), Ok(to), Some(promotion)) =>
                return Role::from_char(promotion).map(|role| {
                    Move::Normal { from, to, promotion: Some(role) }
                }),
            (Ok(from), Ok(to), None) =>
                return Some(Move::Normal { from, to, promotion: None }),
            _ => ()
        }

        match (uci.chars().nth(0), uci.chars().nth(1), Square::from_str(&uci[2..4])) {
            (Some(piece), Some('@'), Ok(to)) =>
                return Role::from_char(piece.to_ascii_lowercase()).map(|role| {
                    Move::Put { role, to }
                }),
            _ => ()
        }

        if uci == "0000" {
            return Some(Move::Null)
        }

        None
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Move::Normal { from, to, promotion: None } =>
                write!(f, "{}{}", from, to),
            Move::Normal { from, to, promotion: Some(promotion) } =>
                write!(f, "{}{}{}", from, to, promotion.char()),
            Move::Put { to, role } =>
                write!(f, "{}@{}", role.char().to_ascii_uppercase(), to),
            Move::Null =>
                write!(f, "0000")
        }
    }
}