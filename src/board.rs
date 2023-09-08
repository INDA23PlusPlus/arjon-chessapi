const BOARD_ROW_COUNT: usize = 8;
const BOARD_COL_COUNT: usize = 8;
const WHITE_KING_STARTING_POS: Position = Position {col: 4, row: 7};
const BLACK_KING_STARTING_POS: Position = Position {col: 4, row: 0};

use std::mem;
use crate::piece::{Color, Piece, PieceType};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoardError {
    IllegalMove,
}

#[derive(Debug, Clone)]
pub struct Board {
    // The representation of the board. Value is Some(Piece) if occupied, otherwise None
    board: [[Option<Piece>; 8]; 8],

    // Useful for checking if king will be attacked after a specific move
    white_king_pos: Position,
    black_king_pos: Position,

    // If castling is still available for black or white. Doesn't mean that it is a legal move
    castle_white: bool,
    castle_black: bool,

    // The column that you are able to do en passant to, or -1 if unavailable
    en_passant_col: i8,

    turn: Color,
}

// Generates a board with the default positions
impl Default for Board {
    fn default() -> Self {
        Self {
            board: [
                // Black pieces
                [
                    Some(Piece { piece_type: PieceType::Rook,   color: Color::Black }),
                    Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
                    Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
                    Some(Piece { piece_type: PieceType::Queen,  color: Color::Black }),
                    Some(Piece { piece_type: PieceType::King,   color: Color::Black }),
                    Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
                    Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
                    Some(Piece { piece_type: PieceType::Rook,   color: Color::Black })
                ],
                [Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }); BOARD_COL_COUNT],
                // Empty rows
                [None; BOARD_COL_COUNT],
                [None; BOARD_COL_COUNT],
                [None; BOARD_COL_COUNT],
                [None; BOARD_COL_COUNT],
                // White pieces
                [Some(Piece { piece_type: PieceType::Pawn, color: Color::White }); BOARD_COL_COUNT],
                [
                    Some(Piece { piece_type: PieceType::Rook,   color: Color::White }),
                    Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
                    Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
                    Some(Piece { piece_type: PieceType::Queen,  color: Color::White }),
                    Some(Piece { piece_type: PieceType::King,   color: Color::White }),
                    Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
                    Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
                    Some(Piece { piece_type: PieceType::Rook,   color: Color::White })
                ],
            ],

            black_king_pos: BLACK_KING_STARTING_POS,
            white_king_pos: WHITE_KING_STARTING_POS,

            castle_black: true,
            castle_white: true,

            en_passant_col: -1,

            turn: Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    col: i8,
    row: i8,
}

// Pass this struct as an argument when you want to make a move
#[derive(Debug, Clone)]
pub struct Move {
    from: Position,
    to: Position,
    promotion: Option<Piece>,
}

impl Board {
    // Generates a new board with the default starting position
    pub fn new() -> Board {
        Default::default()
    }

    fn at(&mut self, pos: &Position) -> &mut Option<Piece> {
        &mut self.board[pos.row as usize][pos.col as usize]
    }

    fn is_own_king_attacked_after_move(&self, mv: &Move) -> bool {
        todo!();
    }

    fn is_legal(&self, mv: &Move) -> bool {
        todo!();
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        todo!();
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

                if (mv.to.col - mv.from.col).abs() >= 2 {
                    *match piece.color {
                        Color::White => &mut self.castle_white,
                        Color::Black => &mut self.castle_black,
                    } = false;
                }
            }

            _ => {}
        };

        // Move the piece into the new position, replacing it with None in the process
        *self.at(&mv.to) = mem::replace(self.at(&mv.from), None);


        Ok(())
    }
}