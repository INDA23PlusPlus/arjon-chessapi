mod board;
mod piece;
extern crate rand;

use board::{Move, Position};

fn to_cordinate(c: char) -> i8 {
    if c >= 'a' && c <= 'h' {
        return c as i8 - 'a' as i8;
    }
    7 - (c as i8 - '1' as i8)
}

fn convert(mv: &str) -> Move {
    let x: Vec<char> = mv.chars().collect();
    Move {
        from: Position {
            row: to_cordinate(x[1]),
            col: to_cordinate(x[0]),
        },
        to: Position {
            row: to_cordinate(x[3]),
            col: to_cordinate(x[2]),
        },
        promotion: None,
    }
}

#[cfg(test)]
mod tests {
    use board::Board;
    use convert;
    use std::fmt::Debug;

    #[test]
    fn checkmate() {
        let moves: Vec<((i8, i8), (i8, i8))> = vec![((0, 0), (0, 0))];
        todo!()
    }

    #[test]
    fn castling() {
        let mut moves: Vec<(&str)> = vec![
            "e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "g8f6", "d2d3", "f8c5", "e1g1", "e8g8",
        ];

        let mut board = Board::new();

        for mv in moves {
            board.print_board();
            println!();
            assert_eq!(board.is_legal(&convert(mv)), true);
            board.make_move(&convert(mv)).unwrap();
        }
        board.print_board();

        moves = vec![
            "e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "g8f6", "d2d3", "f8c5", "b1c3", "d7d6", "c1e3",
            "c8d7", "d1e2", "d8e7", "e1c1", "h7h5", "f3e5", "h5h4", "e5f7",
        ];

        board = Board::new();

        for mv in moves {
            board.print_board();
            println!();
            assert_eq!(board.is_legal(&convert(mv)), true);
            board.make_move(&convert(mv)).unwrap();
        }
        board.print_board();

        assert_eq!(board.is_legal(&convert("e8c8")), false);

        moves = vec!["h8h7", "g2g4", "h4g3", "f2g3"];

        for mv in moves {
            board.print_board();
            println!();
            assert_eq!(board.is_legal(&convert(mv)), true);
            board.make_move(&convert(mv)).unwrap();
        }

        assert_eq!(board.is_legal(&convert("e8g8")), false);
    }
}
