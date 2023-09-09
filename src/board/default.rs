use board::{BLACK_KING_STARTING_POS, Board, BOARD_COL_COUNT, WHITE_KING_STARTING_POS};
use piece::{Color, Piece, PieceType};

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

            long_castle_white:  true,
            short_castle_white: true,
            long_castle_black:  true,
            short_castle_black: true,

            en_passant_col: -1,

            turn: Color::White,
        }
    }
}