// This file is part of the shakmaty library.
// Copyright (C) 2017-2018 Niklas Fiekas <niklas.fiekas@backscattering.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Attack and ray tables.
//!
//! # Example
//!
//! ```
//! # use shakmaty::{Rank, Square, Bitboard};
//! use shakmaty::attacks;
//!
//! let occupied = Bitboard::from(Rank::Sixth); // blocking pieces
//! let attacks = attacks::bishop_attacks(Square::C2, occupied);
//! // . . . . . . . .
//! // . . . . . . . .
//! // 0 0 0 0 0 0 1 0
//! // . . . . . 1 . .
//! // 1 . . . 1 . . .
//! // . 1 . 1 . . . .
//! // . . . . . . . .
//! // . 1 . 1 . . . .
//!
//! assert!(attacks.contains(Square::G6));
//! assert!(!attacks.contains(Square::H7));
//! ```

use square::Square;
use bitboard::Bitboard;
use types::{Color, Piece, Role};
use magics;

include!(concat!(env!("OUT_DIR"), "/attacks.rs")); // generated by build.rs

/// Looks up attacks for a pawn of `color` on `sq`.
#[inline]
pub fn pawn_attacks(color: Color, sq: Square) -> Bitboard {
    // This is safe because properly constructed squares are in bounds.
    Bitboard(match color {
        Color::White => WHITE_PAWN_ATTACKS[usize::from(sq)],
        Color::Black => BLACK_PAWN_ATTACKS[usize::from(sq)],
    })
}

/// Looks up attacks for a knight on `sq`.
#[inline]
pub fn knight_attacks(sq: Square) -> Bitboard {
    Bitboard(KNIGHT_ATTACKS[usize::from(sq)])
}

/// Looks up attacks for a king on `sq`.
#[inline]
pub fn king_attacks(sq: Square) -> Bitboard {
    Bitboard(KING_ATTACKS[usize::from(sq)])
}

/// Looks up attacks for a rook on `sq` with `occupied` squares.
#[inline]
pub fn rook_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    let m = &magics::ROOK_MAGICS[usize::from(sq)];

    // This is safe because the attack table was generated with sufficient size
    // for all relevant occupancies.
    let idx = (m.factor.wrapping_mul(occupied.0 & m.mask) >> (64 - 12)) as usize + m.offset;
    debug_assert!(idx < ATTACKS.len());
    Bitboard(unsafe { *ATTACKS.get_unchecked(idx) })
}

/// Gets the set of potential blocking squares for a rook on `sq`.
///
/// # Example
///
/// ```
/// # use shakmaty::{Square};
/// # use shakmaty::attacks;
/// #
/// let mask = attacks::rook_mask(Square::E8);
/// // 0 1 1 1 0 1 1 0
/// // . . . . 1 . . .
/// // . . . . 1 . . .
/// // . . . . 1 . . .
/// // . . . . 1 . . .
/// // . . . . 1 . . .
/// // . . . . 1 . . .
/// // . . . . 0 . . .
///
/// assert_eq!(mask.count(), 11);
#[inline]
pub fn rook_mask(sq: Square) -> Bitboard {
    Bitboard(magics::ROOK_MAGICS[usize::from(sq)].mask)
}

/// Looks up attacks for a bishop on `sq` with `occupied` squares.
#[inline]
pub fn bishop_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    let m = &magics::BISHOP_MAGICS[usize::from(sq)];

    // This is safe because the attack table was generated with sufficient size
    // for all relevant occupancies.
    let idx = (m.factor.wrapping_mul(occupied.0 & m.mask) >> (64 - 9)) as usize + m.offset;
    debug_assert!(idx < ATTACKS.len());
    Bitboard(unsafe { *ATTACKS.get_unchecked(idx) })
}

/// Gets the set of potential blocking squares for a bishop on `sq`.
///
/// # Example
///
/// ```
/// # use shakmaty::{Square};
/// # use shakmaty::attacks;
/// #
/// let mask = attacks::bishop_mask(Square::D5);
/// // 0 . . . . . 0 .
/// // . 1 . . . 1 . .
/// // . . 1 . 1 . . .
/// // . . . 0 . . . .
/// // . . 1 . 1 . . .
/// // . 1 . . . 1 . .
/// // 0 . . . . . 1 .
/// // . . . . . . . 0
///
/// assert_eq!(mask.count(), 9);
/// ```
#[inline]
pub fn bishop_mask(sq: Square) -> Bitboard {
    Bitboard(magics::BISHOP_MAGICS[usize::from(sq)].mask)
}

/// Looks up attacks for a queen on `sq` with `occupied` squares.
#[inline]
pub fn queen_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    rook_attacks(sq, occupied) ^ bishop_attacks(sq, occupied)
}

/// Looks up attacks for `piece` on `sq` with `occupied` squares.
pub fn attacks(sq: Square, piece: Piece, occupied: Bitboard) -> Bitboard {
    match piece.role {
        Role::Pawn => pawn_attacks(piece.color, sq),
        Role::Knight => knight_attacks(sq),
        Role::Bishop => bishop_attacks(sq, occupied),
        Role::Rook => rook_attacks(sq, occupied),
        Role::Queen => queen_attacks(sq, occupied),
        Role::King => king_attacks(sq),
    }
}

/// The rank, file or diagonal with the two squares (or an empty [`Bitboard`]
/// if they are not aligned).
///
/// # Example
///
/// ```
/// # use shakmaty::attacks;
/// # use shakmaty::Square;
/// #
/// let ray = attacks::ray(Square::E2, Square::G4);
/// // . . . . . . . .
/// // . . . . . . . .
/// // . . . . . . . .
/// // . . . . . . . 1
/// // . . . . . . 1 .
/// // . . . . . 1 . .
/// // . . . . 1 . . .
/// // . . . 1 . . . .
/// ```
///
/// [`Bitboard`]: ../struct.Bitboard.html
#[inline]
pub fn ray(a: Square, b: Square) -> Bitboard {
    Bitboard(BB_RAYS[usize::from(a)][usize::from(b)])
}

/// The squares between the two squares (bounds not included), or an empty
/// [`Bitboard`] if they are not on the same rank, file or diagonal.
///
/// # Example
///
/// ```
/// # use shakmaty::attacks;
/// # use shakmaty::Square;
/// #
/// let between = attacks::between(Square::B1, Square::B7);
/// // . . . . . . . .
/// // . 0 . . . . . .
/// // . 1 . . . . . .
/// // . 1 . . . . . .
/// // . 1 . . . . . .
/// // . 1 . . . . . .
/// // . 1 . . . . . .
/// // . 0 . . . . . .
/// ```
///
/// [`Bitboard`]: ../struct.Bitboard.html
#[inline]
pub fn between(a: Square, b: Square) -> Bitboard {
    Bitboard(BB_BETWEEN[usize::from(a)][usize::from(b)])
}

/// Tests if all three squares are aligned on a rank, file or diagonal.
///
/// # Example
///
/// ```
/// # use shakmaty::attacks;
/// # use shakmaty::Square;
/// #
/// assert!(attacks::aligned(Square::A1, Square::B2, Square::C3));
/// ```
#[inline]
pub fn aligned(a: Square, b: Square, c: Square) -> bool {
    ray(a, b).contains(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_attacks() {
        assert_eq!(rook_attacks(Square::D6, Bitboard(0x3f7f28802826f5b9)),
                   Bitboard(0x8370808000000));
    }
}
