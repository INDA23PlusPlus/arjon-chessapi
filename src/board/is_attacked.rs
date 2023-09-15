use board::*;
use piece::PieceType;

impl Board {
    pub(in board) fn is_attacked(&self, pos: &Position) -> bool {
        self.is_attacked_by_pawn(pos)
            || self.is_attacked_by_knight(pos)
            || self.is_attacked_by_king(pos)
            || self.is_attacked_adjacent(pos)
            || self.is_attacked_diagonal(pos)
    }

    fn is_attacked_by_knight(&self, pos: &Position) -> bool {
        let moves: [[i8; 2]; 4] = [[1, 2], [2, 1], [1, -2], [2, -1]];

        let opponent_knight = Piece {
            piece_type: PieceType::Knight,
            color: self.turn,
        };

        for dir in moves {
            let dpos = Position {
                row: dir[0],
                col: dir[1],
            };
            let pos_a = pos.clone() + dpos;
            let pos_b = pos.clone() - dpos;

            if (pos_a.out_of_bounds() == false
                && at!(self, pos_a)
                    .as_ref()
                    .map_or(false, |piece| piece == &opponent_knight))
                || (pos_b.out_of_bounds() == false
                    && at!(self, pos_b)
                        .as_ref()
                        .map_or(false, |piece| piece == &opponent_knight))
            {
                return true;
            }
        }

        false
    }

    fn is_attacked_by_pawn(&self, pos: &Position) -> bool {
        todo!("Implement a check for if pos is a pawn that can be captured using en passant");
        let dir: i8 = match self.turn {
            Color::White => -1,
            Color::Black => 1,
        };
        let attack_a = *pos + Position { row: dir, col: -1 };
        let attack_b = *pos + Position { row: dir, col: 1 };

        let opponent_pawn = Piece {
            piece_type: PieceType::Pawn,
            color: self.turn,
        };

        (attack_a.out_of_bounds() == false
            && at!(self, attack_a)
                .as_ref()
                .map_or(false, |piece| piece == &opponent_pawn))
            || (attack_b.out_of_bounds() == false
                && at!(self, attack_b)
                    .as_ref()
                    .map_or(false, |piece| piece == &opponent_pawn))
    }

    fn is_attacked_by_king(&self, pos: &Position) -> bool {
        let king_pos = &match self.turn {
            Color::White => self.white_king_pos,
            Color::Black => self.black_king_pos,
        };

        let dpos = (*pos - *king_pos).abs();
        dpos.row <= 1 && dpos.col <= 1
    }

    fn is_attacked_adjacent(&self, pos: &Position) -> bool {
        let dirs: [[i8; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        let types = [PieceType::Rook, PieceType::Queen];
        self.is_attacked_line(pos, &dirs, &types)
    }

    fn is_attacked_diagonal(&self, pos: &Position) -> bool {
        let dirs: [[i8; 2]; 4] = [[1, 1], [-1, -1], [1, -1], [-1, 1]];
        let types = [PieceType::Bishop, PieceType::Queen];
        self.is_attacked_line(pos, &dirs, &types)
    }

    fn is_attacked_line(
        &self,
        pos: &Position,
        dirs: &[[i8; 2]; 4],
        types: &[PieceType; 2],
    ) -> bool {
        for dir in dirs {
            let dpos = Position {
                row: dir[0],
                col: dir[1],
            };
            let mut pos = *pos + dpos;

            while pos.out_of_bounds() == false {
                if let Some(piece) = at!(self, pos).as_ref() {
                    if piece.color == self.turn && types.contains(&piece.piece_type) {
                        return true;
                    }
                    break;
                }

                pos += dpos;
            }
        }

        false
    }
}
