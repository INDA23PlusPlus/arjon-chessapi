use board::*;
use piece::PieceType;

impl Board {
    #[inline]
    pub fn retain_legal_capturing(&self, moves: Vec<Move>) -> Vec<Move> {
        self.retain_legal(self.retain_capturing(moves))
    }
    #[inline]
    pub fn retain_legal(&self, mut moves: Vec<Move>) -> Vec<Move> {
        moves.retain(|mv| self.is_legal(mv));
        moves
    }
    #[inline]
    pub fn retain_capturing(&self, mut moves: Vec<Move>) -> Vec<Move> {
        moves.retain(|mv| !mv.to.out_of_bounds() && at!(self, mv.to).is_some());
        moves
    }
    fn is_own_king_attacked_after_move(&self, mv: &Move) -> bool {
        let mut board_copy = self.clone();
        board_copy.unsafe_make_move(mv).unwrap();

        board_copy.is_attacked_by_player(&match self.turn {
            Color::White => board_copy.white_king_pos,
            Color::Black => board_copy.black_king_pos,
        })
    }
    fn is_legal_pawn(&self, mv: &Move) -> bool {
        let dcol = (mv.to.col - mv.from.col).abs();
        let drow = mv.to.row - mv.from.row;

        // If promotion necessary
        if (mv.to.row == 0 || mv.to.row == BOARD_ROW_COUNT as i8 - 1)
            && (mv
                .promotion
                .as_ref()
                .map_or(true, |piece_type| *piece_type == PieceType::Pawn))
        {
            return false;
        }

        let dir: i8 = match self.turn {
            Color::White => -1,
            Color::Black => 1,
        };

        // 1 move forward
        if dcol == 0 && drow == dir {
            return at!(self, mv.to) == None;
        }

        // 2 moves forward
        if dcol == 0 && drow == 2 * dir {
            let start_row = match self.turn {
                Color::White => WHITE_PAWN_STARTING_ROW,
                Color::Black => BLACK_PAWN_STARTING_ROW,
            };
            // Return (pawn is on starting row) && (two squares in front are free)
            return start_row == mv.from.row
                && at!(self, (mv.from + mv.to) / 2) == None
                && at!(self, mv.to) == None;
        }

        // Capture (normal capture or en passant)
        if dcol == 1 && drow == dir {
            return at!(self, mv.to)
                .as_ref()
                .map_or(false, |p| p.color != self.turn)
                || (self.en_passant_col == mv.to.col
                    && mv.from.row
                        == match self.turn {
                            Color::White => WHITE_EN_PASSANT_FROM_ROW,
                            Color::Black => BLACK_EN_PASSANT_FROM_ROW,
                        });
        }
        false
    }

    fn is_legal_knight(&self, mv: &Move) -> bool {
        let dpos = (mv.from - mv.to).abs();
        // Make sure that it moves 2 squares in one direction and 1 square in the other
        // and that the target square isn't occupied by a piece of the same color
        ((dpos.col == 1 && dpos.row == 2) || (dpos.col == 2 && dpos.row == 1))
            && at!(self, mv.to).as_ref().map_or(
                true, // Unoccupied square is fine :)
                |target| target.color != self.turn,
            )
    }

    fn is_legal_rook(&self, mv: &Move) -> bool {
        let dpos = mv.to - mv.from;

        // Check that the move is either horizontal or vertical
        if (dpos.col == 0) == (dpos.row == 0) {
            return false;
        }

        // Number of steps needed to reach the target
        let steps = (dpos.col + dpos.row).abs();
        // The difference in position that one step resemblances
        let step = dpos / steps;

        let mut curr_pos = mv.from.clone();

        for _ in 0..(steps - 1) {
            curr_pos += step;
            if at!(self, curr_pos) != None {
                return false;
            }
        }

        at!(self, mv.to)
            .as_ref()
            .map_or(true, |piece| piece.color != self.turn)
    }

    fn is_legal_bishop(&self, mv: &Move) -> bool {
        let dpos = mv.to - mv.from;

        // Bishop has to move the same amount of columns as rows
        if dpos.col.abs() != dpos.row.abs() {
            return false;
        }
        let steps = dpos.col.abs();

        let step = dpos / steps;

        let mut curr_pos = mv.from.clone();

        for _ in 0..(steps - 1) {
            curr_pos += step;
            if at!(self, curr_pos) != None {
                return false;
            }
        }
        at!(self, mv.to)
            .as_ref()
            .map_or(true, |piece| piece.color != self.turn)
    }

    fn is_legal_queen(&self, mv: &Move) -> bool {
        // Queen can move as rook and as bishop
        self.is_legal_bishop(mv) || self.is_legal_rook(mv)
    }

    fn is_legal_king(&self, mv: &Move) -> bool {
        let dpos = (mv.to - mv.from).abs();

        // Normal move
        if dpos.col <= 1 && dpos.row <= 1 {
            return at!(self, mv.to)
                .as_ref()
                .map_or(true, |piece| piece.color != self.turn);
        }

        // Castling
        let long_castle = match self.turn {
            Color::White => self.long_castle_white,
            Color::Black => self.long_castle_black,
        };
        let short_castle = match self.turn {
            Color::White => self.short_castle_white,
            Color::Black => self.short_castle_black,
        };

        let king_pos = match self.turn {
            Color::White => self.white_king_pos,
            Color::Black => self.black_king_pos,
        };

        if dpos.col == 2 && dpos.row == 0 {
            let intermediary_pos = (mv.from + mv.to) / 2;

            // Castling is not legal if this square is attacked
            if self.is_attacked_by_opponent(&intermediary_pos) {
                return false;
            }
            // Short castle moves toward higher col counts
            let is_short_castle = (mv.to.col - mv.from.col) > 0;
            let rook_pos: Position;

            if is_short_castle {
                if !short_castle {
                    return false;
                }
                rook_pos = match self.turn {
                    Color::White => WHITE_ROOK_SHORT_STARTING_POS,
                    Color::Black => BLACK_ROOK_SHORT_STARTING_POS,
                };
            } else
            /* is_long_castle */
            {
                if !long_castle {
                    return false;
                }
                rook_pos = match self.turn {
                    Color::White => WHITE_ROOK_LONG_STARTING_POS,
                    Color::Black => BLACK_ROOK_LONG_STARTING_POS,
                };
            }
            // get_moves_adjacent()
            // will return all possible moves without checking
            // if it is actually legal to capture your own king.
            // This will ensure that there is a clear path between the rook and the king
            return self
                .generate_moves_line(&rook_pos, &vec![[0, 1], [0, -1]])
                .contains(&Move {
                    from: rook_pos,
                    to: king_pos,
                    promotion: None,
                });
        }
        false
    }

    pub(crate) fn is_legal(&self, mv: &Move) -> bool {
        // Check that the move is within the chessboard
        if mv.from.out_of_bounds() || mv.to.out_of_bounds() {
            return false;
        }
        // Not moving is not a valid move
        if mv.from == mv.to {
            return false;
        }

        let piece = match at!(self, mv.from).as_ref() {
            Some(piece) => piece,
            None => return false, // No piece at the starting position
        };
        if piece.color != self.turn {
            return false;
        }

        (match piece.piece_type {
            PieceType::Pawn => self.is_legal_pawn(mv),
            PieceType::Rook => self.is_legal_rook(mv),
            PieceType::Knight => self.is_legal_knight(mv),
            PieceType::Bishop => self.is_legal_bishop(mv),
            PieceType::Queen => self.is_legal_queen(mv),
            PieceType::King => self.is_legal_king(mv),
        } && !self.is_own_king_attacked_after_move(mv))
    }
}
