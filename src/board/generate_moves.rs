use board::*;
use piece::PieceType;

impl Board {
    pub fn generate_legal_moves(&self) -> Vec<Move> {
        self.retain_legal(self.generate_moves())
    }

    pub fn generate_legal_capturing_moves(&self) -> Vec<Move> {
        self.retain_legal_capturing(self.generate_moves())
    }

    fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for row in 0..BOARD_ROW_COUNT {
            for col in 0..BOARD_COL_COUNT {
                moves.append(&mut self.generate_moves_piece(&Position {
                    row: row as i8,
                    col: col as i8,
                }));
            }
        }
        moves
    }

    fn generate_moves_piece(&self, pos: &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        if let Some(piece) = at!(self, pos).as_ref() {
            moves = match piece.piece_type {
                PieceType::Pawn => self.generate_moves_pawn(pos),
                PieceType::Rook => self.generate_moves_adjacent(pos),
                PieceType::Knight => self.generate_moves_knight(pos),
                PieceType::Bishop => self.generate_moves_diagonal(pos),
                PieceType::Queen => [
                    self.generate_moves_diagonal(pos),
                    self.generate_moves_adjacent(pos),
                ]
                .concat(),
                PieceType::King => self.generate_moves_king(pos),
            };
        }

        moves
    }

    pub(crate) fn generate_moves_knight(&self, from: &Position) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        // Directions we can move upwards,
        // we will generate the downwards moves by subtracting these values
        let dirs: [[i8; 2]; 4] = [[1, 2], [2, 1], [1, -2], [2, -1]];

        for dir in dirs {
            let dpos = Position {
                row: dir[0],
                col: dir[1],
            };
            moves.push(Move {
                from: *from,
                to: *from + dpos,
                promotion: None,
            });
            moves.push(Move {
                from: *from,
                to: *from - dpos,
                promotion: None,
            });
        }

        moves
    }

    pub(crate) fn generate_moves_pawn(&self, from: &Position) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let dir = Board::dir(&self.turn);

        let positions = [
            // Non capturing moves
            Position {
                row: from.row + dir,
                col: from.col,
            },
            Position {
                row: from.row + 2 * dir,
                col: from.col,
            },
            // Capturing moves
            Position {
                row: from.row + dir,
                col: from.col + 1,
            },
            Position {
                row: from.row + dir,
                col: from.col - 1,
            },
        ];

        for position in positions {
            moves.push(Move {
                from: *from,
                to: position,
                // Defaults to queen promotion so we don't have
                // to generate a move for every promotion
                promotion: Some(PieceType::Queen),
            });
        }
        moves
    }

    pub(crate) fn generate_moves_king(&self, from: &Position) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // Normal moves
        for row in -1i8..=1i8 {
            for col in -1i8..=1i8 {
                moves.push(Move {
                    from: *from,
                    to: *from + Position { row, col },
                    promotion: None,
                });
            }
        }
        // Castling
        moves.push(Move {
            from: *from,
            to: *from + Position { row: 0, col: 2 },
            promotion: None,
        });
        moves.push(Move {
            from: *from,
            to: *from + Position { row: 0, col: -2 },
            promotion: None,
        });
        moves
    }

    pub(crate) fn generate_moves_adjacent(&self, pos: &Position) -> Vec<Move> {
        let dirs = vec![[1, 0], [0, 1], [-1, 0], [0, -1]];
        self.generate_moves_line(pos, &dirs)
    }

    pub(crate) fn generate_moves_diagonal(&self, pos: &Position) -> Vec<Move> {
        let dirs = vec![[1, 1], [-1, -1], [1, -1], [-1, 1]];
        self.generate_moves_line(pos, &dirs)
    }
    pub(crate) fn generate_moves_line(&self, from: &Position, dirs: &Vec<[i8; 2]>) -> Vec<Move> {
        let mut moves = Vec::new();
        for dir in dirs {
            // If both drow and dcol were 0 then we would become stuck in a loop
            if dir[0] == 0 && dir[1] == 0 {
                continue;
            }

            let dpos = Position {
                row: dir[0],
                col: dir[1],
            };
            let mut to = *from + dpos;

            while to.out_of_bounds() == false {
                moves.push(Move {
                    from: *from,
                    to: to,
                    promotion: None,
                });

                // If we have encountered a piece then we will not be able to move any further
                if at!(self, to).is_some() {
                    break;
                }

                to += dpos;
            }
        }

        moves
    }
}
