# Arvid Jonasson Chess API

## How To Use
* Create a chessboard using `let mut board = Board::new();`
* Generate legal moves using `let moves = board.generate_legal_moves();`.
  * This returns a `Vec<Move>` of all moves available to be played from the current position.
  * `Move` is a struct consisting of the members `from: Position`, `to: Position` and `promotion: Option<PieceType>`.
    * `from` is the position of the piece that will be moved and `to` is the square that the piece will be moved to.
    * `promotion` needs to be set to `Some(PieceType)` when a pawn is moving to the last rank. Promoting to a pawn will result in an error.
      * `PieceType` is an enum containing all possible chess roles, e.g. `Pawn`, `King`, and so on...
  * `Position` is a struct consisting of the members `row: i8` and `col: i8`. Both are in the range of 0 to 7 inclusive.
    * `row = 0` corresponds to row **8** on a chessboard and `row = 7` corresponds to row **1**.
    * `col = 0` corresponds to column **A** on a chessboard and `col = 7` corresponds to column **H**.
* Check if a move is legal by calling `board.is_legal(mv)` where `mv` is of type `Move`.
* Get the current color to play by calling `board.whose_turn()`. This will return a value of type `Color` set to the color of the player that is to play the next move.
* Use `board.get_board()` to get a copy of the current board state. This returns a `[[SquareType; 8]; 8]`.
  * `SquareType` is a typedef for `Option<Piece>`, `Piece` is a struct containing the members `piece_type: PieceType` and `color: Color`.
  * `PieceType` is described above. `Color` is an enum consisting of `White` or `Black`.
  * The board uses row for its first index and column for its second index, i.e. `board[row][column]`.
  * If the cell is `None` then the square is not occupied, and accordingly `Some(Piece)` means that it is occupied by a piece with the attributes of `Piece`.
* Make move by calling `board.make_move(mv)` where `mv` is of type `Move`.
  * This will execute the move and return `Ok(())` if the move is legal and refuse to execute the move and return `Err(BoardError::IllegalMove)` otherwise.
  * Castling is an ordinary kings move. Set `from` to its current position and `to` to its position after castling.
* Use `board.is_stalemate()` to check for stalemate and `board.is_checkmate()` to check for checkmate.
  * If `board.is_checkmate()` returns true then the player that made the last move (the checkmate) won.
* Finally reset the board by calling `board = Board::new();`