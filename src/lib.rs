#[macro_use]
mod board;
mod piece;
extern crate rand;

use board::{Move, Position};

#[inline]
fn to_cordinate(c: char) -> i8 {
    if c >= 'a' && c <= 'h' {
        return c as i8 - 'a' as i8;
    }
    7 - (c as i8 - '1' as i8)
}

#[inline]
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
    use board::*;
    use piece::PieceType;
    use std::fmt::Debug;
    use std::mem::swap;
    use {board, convert};

    // Test according to Shannon number
    #[test]
    fn test() {
        let mut prev: Vec<Board> = vec![Board::new()];
        let mut curr: Vec<Board> = vec![];
        let expected = vec![20, 400, 8902, 197281, 4865609];
        let ep_expected = vec![0, 0, 0, 0, 258];
        let cm_expected = vec![0, 0, 0, 8, 347];

        let mut ep;
        let mut cm;

        let n = 5;
        for current in 0..n {
            cm = 0;
            ep = 0;
            println!("Test {}.", current);
            curr = Vec::new();
            for board in &prev {
                let moves = board.generate_legal_moves();
                for mv in moves {
                    let mut x = board.clone();
                    if at!(board, mv.from).as_ref().unwrap().piece_type == PieceType::Pawn {
                        if mv.to.col - mv.from.col != 0 && at!(board, mv.to).as_ref().is_none() {
                            ep += 1;
                        }
                    }
                    x.make_move(&mv).unwrap();
                    if x.is_checkmate() {
                        cm += 1;
                    }
                    curr.push(x);
                }
            }
            swap(&mut prev, &mut curr);
            println!("No. of cm: {}", cm);
            println!("No. of e.p.: {}", ep);
            println!("Generated moves = {}", prev.len());
            println!("Expected no. of moves = {}", expected[current]);

            assert_eq!(cm, cm_expected[current]);
            assert_eq!(ep, ep_expected[current]);

            // I know this is 35 leaves (0.0007%) off att ply 5.
            // Haven't figured out the problem yet.
            // I might be counting the leaves incorrectly.
            assert_eq!(prev.len(), expected[current]);
            println!();
        }
    }

    #[test]
    fn game_with_en_passant_castling_and_checkmate() {
        let mut moves: Vec<&str> = vec![
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

        assert_eq!(board.is_legal(&convert("c5e3")), true);
        board.make_move(&convert("c5e3")).unwrap();
        assert_eq!(board.generate_legal_moves().len(), 4);

        board.make_move(&convert("c1b1")).unwrap();

        assert_eq!(board.is_legal(&convert("e8f7")), false);

        board.make_move(&convert("h7h5")).unwrap();
        board.make_move(&convert("e2h5")).unwrap();
        board.make_move(&convert("a7a5")).unwrap();
        board.make_move(&convert("f7d6")).unwrap();
        assert_eq!(board.generate_legal_moves().len(), 2);
        board.make_move(&convert("e8f8")).unwrap();
        board.make_move(&convert("h5g6")).unwrap();
        assert_eq!(board.is_legal(&convert("b7b7")), false);
        board.make_move(&convert("b7b6")).unwrap();
        board.make_move(&convert("h2h4")).unwrap();
        board.make_move(&convert("b6b5")).unwrap();
        board.make_move(&convert("h4h5")).unwrap();
        board.make_move(&convert("b5b4")).unwrap();
        board.make_move(&convert("h5h6")).unwrap();
        board.make_move(&convert("b4b3")).unwrap();
        board.make_move(&convert("h6h7")).unwrap();
        board.make_move(&convert("a5a4")).unwrap();
        let mut temp = convert("h7h8");
        temp.promotion = Some(PieceType::Queen);
        board.make_move(&temp).unwrap();
        board.print_board();
        assert_eq!(board.generate_legal_moves().len(), 1);
        board.make_move(&board.generate_legal_moves()[0]).unwrap();
        board.print_board();
        board.make_move(&convert("h8g8")).unwrap();
        assert_eq!(board.generate_legal_moves().len(), 0);
        assert_eq!(board.is_checkmate(), true);
        assert_eq!(board.is_stalemate(), false);
    }
}
