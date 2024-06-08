use p22::calc::*;
use p22::figures::*;
use p22::tictac::*;

/// Calculator tests

#[test]
fn test_celsius2farenheit() {
    assert_eq!(celsius2farenheit(10), 50);
}
#[test]
fn test_farenheit2celsius() {
    assert_eq!(farenheit2celsius(50), 10);
}
#[test]
fn test_fibonacci_loop() {
    assert_eq!(fibonacci_loop(6), 8);
}
#[test]
fn test_fibonacci_rec() {
    assert_eq!(fibonacci_rec(6), 8);
}

/// Figures tests

#[test]
fn test_point() {
    // use Shape::Point;
    let a = Point { x: 6.0, y: 6.0 };
    let b = Point { x: 7.0, y: 7.0 };

    assert_eq!(point_distance(a, b), point_distance(b, a));
    assert_ne!(point_distance(a, a), point_distance(b, a));
}
#[test]
fn test_circle() {
    let center_point = Point { x: 6.0, y: 6.0 };
    let radius = 10.0;

    let circ = Circle {
        center_point,
        radius,
    };
    assert_eq!(circle_area_perimeter(circ), (314.15927, 62.831856));

    let radius = 1.0;
    let circ = Circle {
        center_point,
        radius,
    };
    assert_eq!(
        circle_area_perimeter(circ),
        (std::f32::consts::PI, 6.2831855)
    );

    let radius = 0.0;
    let circ = Circle {
        center_point,
        radius,
    };
    assert_eq!(circle_area_perimeter(circ), (0.0, 0.0));
}
#[test]
fn test_triangle() {
    let a = Point { x: 0.0, y: 1.0 };
    let b = Point { x: 3.0, y: 4.0 };
    let c = Point { x: 6.0, y: 1.0 };

    let trian = Triangle { a, b, c };
    assert_eq!(triangle_area_perimeter(trian), (8.999999, 14.485281));

    let a = Point { x: 5.0, y: 1.0 };
    let b = Point { x: 8.0, y: 4.0 };
    let c = Point { x: 11.0, y: 1.0 };

    let trian = Triangle { a, b, c };
    assert_eq!(triangle_area_perimeter(trian), (8.999999, 14.485281));
}
#[test]
fn test_rectangle() {
    let a = Point { x: 1.0, y: 8.0 };
    let b = Point { x: 5.0, y: 8.0 };
    let c = Point { x: 5.0, y: 3.0 };
    // let d = Point { x: 1.0, y: 3.0 };

    let rectangle = Rectangle { a, b, c };
    assert_eq!(rectangle_area_perimeter(rectangle), (20.0, 18.0));

    let a = Point { x: 1.0, y: 16.0 };
    let b = Point { x: 5.0, y: 16.0 };
    let c = Point { x: 5.0, y: 6.0 };
    // let d = Point { x: 1.0, y: 6.0 };

    let rectangle = Rectangle { a, b, c };
    assert_eq!(rectangle_area_perimeter(rectangle), (40.0, 28.0));

    let a = Point { x: 0.0, y: 6.0 };
    let b = Point { x: 8.0, y: 0.0 };
    let c = Point { x: 20.0, y: 16.0 };
    // let d = Point { x: 12.0, y: 22.0 };

    let rectangle = Rectangle { a, b, c };
    assert_eq!(rectangle_area_perimeter(rectangle), (200.0, 60.0));
}

/// Tests ti tac toe

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
