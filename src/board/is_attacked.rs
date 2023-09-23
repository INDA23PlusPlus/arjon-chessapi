use board::*;
use piece::PieceType;

impl Board {
    // Use this method when you want to check which pieces
    // the opponent will attack after you've made your move
    pub(in board) fn is_attacked_by_opponent(&self, pos: &Position) -> bool {
        self.is_attacked(pos, &self.turn.clone().flip())
    }
    // Use this method when you want to check which pieces
    // are currently under attack. E.g. white queen is under
    // attack by white king when no move has been made yet
    pub(in board) fn is_attacked_by_player(&self, pos: &Position) -> bool {
        self.is_attacked(pos, &self.turn)
    }
    pub(in board) fn is_attacked(&self, pos: &Position, by_color: &Color) -> bool {
        self.is_attacked_by_pawn(pos, by_color)
            || self.is_attacked_by_knight(pos, by_color)
            || self.is_attacked_by_king(pos, by_color)
            || self.is_attacked_adjacent(pos, by_color)
            || self.is_attacked_diagonal(pos, by_color)
    }

    fn is_attacked_by_knight(&self, pos: &Position, by_color: &Color) -> bool {
        let opponent_knight = Piece {
            piece_type: PieceType::Knight,
            color: *by_color,
        };
        let mut moves = self.retain_capturing(self.generate_moves_knight(pos));
        moves.retain(|mv| at!(self, mv.to).as_ref().unwrap() == &opponent_knight);

        moves.len() != 0
    }

    fn is_attacked_by_pawn(&self, pos: &Position, by_color: &Color) -> bool {
        let dir: i8 = match by_color {
            Color::White => -1,
            Color::Black => 1,
        };

        let mut attacks = vec![
            *pos + Position { row: dir, col: -1 },
            *pos + Position { row: dir, col: 1 },
        ];

        let opponent_pawn = Piece {
            piece_type: PieceType::Pawn,
            color: *by_color,
        };

        if pos.col == self.en_passant_col
            && pos.row
                == match by_color {
                    Color::White => WHITE_EN_PASSANT_FROM_ROW,
                    Color::Black => BLACK_EN_PASSANT_FROM_ROW,
                }
        {
            attacks.append(&mut vec![
                *pos + Position { row: 0, col: -1 },
                *pos + Position { row: 0, col: 1 },
            ]);
        }

        for attack in attacks {
            if attack.out_of_bounds() {
                continue;
            }
            if at!(self, attack)
                .as_ref()
                .map_or(false, |piece| piece == &opponent_pawn)
            {
                return true;
            }
        }
        false
    }

    fn is_attacked_by_king(&self, pos: &Position, by_color: &Color) -> bool {
        let king_pos = &match by_color {
            Color::White => self.white_king_pos,
            Color::Black => self.black_king_pos,
        };

        let dpos = (*pos - *king_pos).abs();
        dpos.row <= 1 && dpos.col <= 1
    }

    fn is_attacked_adjacent(&self, pos: &Position, by_color: &Color) -> bool {
        let dirs: Vec<[i8; 2]> = vec![[1, 0], [0, 1], [-1, 0], [0, -1]];
        let types = vec![PieceType::Rook, PieceType::Queen];
        self.is_attacked_line(pos, &dirs, &types, by_color)
    }

    fn is_attacked_diagonal(&self, pos: &Position, by_color: &Color) -> bool {
        let dirs: Vec<[i8; 2]> = vec![[1, 1], [-1, -1], [1, -1], [-1, 1]];
        let types = vec![PieceType::Bishop, PieceType::Queen];
        self.is_attacked_line(pos, &dirs, &types, by_color)
    }

    fn is_attacked_line(
        &self,
        pos: &Position,
        dirs: &Vec<[i8; 2]>,
        types: &Vec<PieceType>,
        by_color: &Color,
    ) -> bool {
        let mut moves = self.retain_capturing(self.generate_moves_line(pos, dirs));
        moves.retain(|mv| {
            at!(self, mv.to)
                .as_ref()
                .map(|piece| piece.color == *by_color && types.contains(&piece.piece_type))
                .unwrap()
        });

        moves.len() != 0
    }
}
