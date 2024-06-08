#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    MoveOutBoard,
    PositionTaken,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StateBoard {
    X,
    O,
    F,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StateGame {
    WinX,
    WinO,
    WinBoth,
    GameOn,
}
#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub mark: StateBoard,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TicTacField {
    pub board: [StateBoard; 9],
}
/// Analyze the field to know if someone won
/// ```
/// use p22::tictac::*;
///let board = [
/// StateBoard::X,
/// StateBoard::O,
/// StateBoard::X,
/// StateBoard::O,
/// StateBoard::X,
/// StateBoard::O,
/// StateBoard::O,
/// StateBoard::O,
/// StateBoard::X,
/// ];
/// let game = TicTacField { board };
/// assert_eq!(analyze(game), StateGame::WinX);
/// ```
pub fn analyze(field: TicTacField) -> StateGame {
    let mut result = StateBoard::F;
    for i in 0..3 {
        if field.board[i] == field.board[i + 3] && field.board[i] == field.board[i + 6] {
            result = field.board[i];
        }
        let offset = i * 3;
        if field.board[offset] == field.board[offset + 1]
            && field.board[offset] == field.board[offset + 2]
        {
            result = field.board[i];
        }
    }
    if field.board[0] == field.board[4] && field.board[0] == field.board[8] {
        result = field.board[0];
    }
    if field.board[2] == field.board[4] && field.board[2] == field.board[6] {
        result = field.board[2];
    }
    match result {
        StateBoard::X => StateGame::WinX,
        StateBoard::O => StateGame::WinO,
        StateBoard::F => {
            if field.board.iter().all(|&x| x != StateBoard::F) {
                StateGame::WinBoth
            } else {
                StateGame::GameOn
            }
        }
    }
}
/// Performs the player's moves if are valid
/// ```
/// use p22::tictac::*;
/// let board = [
///     StateBoard::F,
///     StateBoard::O,
///     StateBoard::X,
///     StateBoard::O,
///     StateBoard::X,
///     StateBoard::O,
///     StateBoard::O,
///     StateBoard::O,
///     StateBoard::X,
/// ];
/// let game = TicTacField { board };
///
/// let player1 = Player {
///     mark: StateBoard::X,
/// };
/// assert_ne!(make_move(game, 0, 0, player1).unwrap(), game);
/// ```
pub fn make_move(field: TicTacField, x: u32, y: u32, player: Player) -> Result<TicTacField, Error> {
    if x > 2 || y > 2 {
        return Err(Error::MoveOutBoard);
    }
    let mut field_mut = field;
    let position: usize = ((x * 3) + y) as usize;
    if field_mut.board[position] != StateBoard::F {
        return Err(Error::PositionTaken);
    }
    field_mut.board[position] = player.mark;
    Ok(field_mut)
}

#[cfg(test)]
mod tests {
    use crate::tictac::*;
    #[test]
    fn test_analyze_win_x() {
        //  WinX
        let board = [
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::O,
            StateBoard::O,
            StateBoard::X,
        ];
        let game = TicTacField { board };
        assert_eq!(analyze(game), StateGame::WinX);
    }
    #[test]
    fn test_analyze_win_y() {
        // WinO
        let board = [
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
        ];
        let game = TicTacField { board };
        assert_eq!(analyze(game), StateGame::WinO);
    }
    #[test]
    fn test_analyze_game_on() {
        // GameOn
        let board = [
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::F,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::X,
        ];
        let game = TicTacField { board };
        assert_eq!(analyze(game), StateGame::GameOn);
    }
    #[test]
    fn test_analyze_win_both() {
        // WinBoth
        let board = [
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::X,
            StateBoard::O,
            StateBoard::O,
            StateBoard::O,
            StateBoard::X,
        ];
        let game = TicTacField { board };
        assert_eq!(analyze(game), StateGame::WinBoth);
    }
    #[test]
    fn test_make_move_success_move() {
        //  Almost WinX
        let board = [
            StateBoard::F,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::O,
            StateBoard::O,
            StateBoard::X,
        ];
        let board_expected = [
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::O,
            StateBoard::O,
            StateBoard::X,
        ];
        let game = TicTacField { board };
        let game_expected = TicTacField {
            board: board_expected,
        };

        let player1 = Player {
            mark: StateBoard::X,
        };

        assert_ne!(make_move(game, 0, 0, player1).unwrap(), game);
        assert_eq!(make_move(game, 0, 0, player1).unwrap(), game_expected);
    }
    #[test]
    fn test_make_move_invalid_move() {
        //  Almost WinX
        let board = [
            StateBoard::F,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::O,
            StateBoard::O,
            StateBoard::X,
        ];
        let game = TicTacField { board };

        let player1 = Player {
            mark: StateBoard::X,
        };

        let result = make_move(game, 2, 8, player1);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::MoveOutBoard);
    }
    #[test]
    fn test_make_move_position_taken() {
        //  Almost WinX
        let board = [
            StateBoard::F,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::X,
            StateBoard::O,
            StateBoard::O,
            StateBoard::O,
            StateBoard::X,
        ];
        let game = TicTacField { board };

        let player1 = Player {
            mark: StateBoard::X,
        };

        let result = make_move(game, 2, 0, player1);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::PositionTaken);
    }
}
