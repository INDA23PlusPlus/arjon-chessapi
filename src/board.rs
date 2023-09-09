mod default;
pub const BOARD_ROW_COUNT: usize = 8;
pub const BOARD_COL_COUNT: usize = 8;
pub(crate) const WHITE_KING_STARTING_POS: Position = Position {col: 4, row: 7};
pub(crate) const BLACK_KING_STARTING_POS: Position = Position {col: 4, row: 0};

use std::mem;
use crate::piece::{Color, Piece, PieceType};

pub type SquareType = Option<Piece>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoardError {
    IllegalMove,
}

#[derive(Debug, Clone)]
pub struct Board {
    // The representation of the board. Value is Some(Piece) if occupied, otherwise None
    board: [[SquareType; 8]; 8],

    // Useful for checking if king will be attacked after a specific move
    white_king_pos: Position,
    black_king_pos: Position,

    // If castling is still available for black or white. Doesn't mean that it is a legal move
    short_castle_white: bool,
    long_castle_white:  bool,
    short_castle_black: bool,
    long_castle_black:  bool,

    // The column that you are able to do en passant to, or -1 if unavailable
    en_passant_col: i8,

    turn: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    row: i8,
    col: i8,
}

// Pass this struct as an argument when you want to make a move
#[derive(Debug, Clone)]
pub struct Move {
    from: Position,
    to: Position,
    promotion: Option<Piece>,
}

// Used to get mutable and immutable access under the same function
trait BoardAccess {
    type T;
    fn at(self, pos: &Position) -> Self::T;
}

impl<'a> BoardAccess for &'a Board {
    type T = &'a SquareType;
    fn at(self, pos: &Position) -> Self::T {
        &self.board[pos.row as usize][pos.col as usize]
    }
}

impl<'a> BoardAccess for &'a mut Board {
    type T = &'a mut SquareType;
    fn at(self, pos: &Position) -> Self::T {
        &mut self.board[pos.row as usize][pos.col as usize]
    }
}

impl Board {
    // Generates a new board with the default starting position
    pub fn new() -> Board {
        Default::default()
    }

    fn is_own_king_attacked_after_move(&self, mv: &Move) -> bool {
        todo!();
    }

    fn is_legal(&self, mv: &Move) -> bool {
        if *self.at(&mv.from) == None { return false; }

        let piece = &self.at(&mv.from).unwrap();
        let piece2 = &self.at(&mv.from).unwrap();
        if piece.color != self.turn { return false; }



        todo!();
    }

    fn get_legal_moves_piece(&self, pos: &Position) -> Vec<Move> {
        todo!();
    }

    /*
     * Iterate through a all squares and and call get_legal_moves_piece() on every square that
     * holds a piece of the correct color
     */
    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for row in 0..BOARD_ROW_COUNT {
            for col in 0..BOARD_COL_COUNT {
                if self.board[row][col] == None { continue; }
                if self.board[row][col].unwrap().color != self.turn { continue; }
                moves.append(
                    &mut self.get_legal_moves_piece(
                        &Position { row: row as i8, col: col as i8 }
                    )
                );
            }
        }
        moves
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), BoardError> {

        if self.is_legal(&mv) == false {
            return Err(BoardError::IllegalMove);
        }
        // Basically flipping the value
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        // Reset en passant
        self.en_passant_col = -1;

        let piece = self.at(&mv.from).unwrap();

        match piece.piece_type {
            PieceType::Pawn => {
                // Flag en passant as available for the next move if pawn moved 2 steps
                if (mv.to.row - mv.from.row).abs() == 2 {
                    self.en_passant_col = mv.to.col;
                }
                // Promote pawn
                else if (mv.to.row == 0) || (mv.to.row == (BOARD_ROW_COUNT as i8) - 1) {
                    *self.at(&mv.from) = mv.promotion;
                }
            }

            PieceType::King => {
                // Update king position
                *match piece.color {
                    Color::White => &mut self.white_king_pos,
                    Color::Black => &mut self.black_king_pos,
                } = mv.to;

                // If king move, mark castling as unavailable
                *match piece.color {
                    Color::White => &mut self.short_castle_white,
                    Color::Black => &mut self.short_castle_black,
                } = false;
                *match piece.color {
                    Color::White => &mut self.long_castle_white,
                    Color::Black => &mut self.long_castle_black,
                } = false;
            }

            _ => {}
        };

        // Move the piece into the new position, replacing it with None in the process
        *self.at(&mv.to) = mem::replace(self.at(&mv.from), None);


        Ok(())
    }
}