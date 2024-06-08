use p22::calc::*;
use p22::figures::*;
use p22::tictac::*;

pub mod song;

fn main() {
    println!("\n\nCalculator\n");

    let x = celsius2farenheit(10i32);
    let y = farenheit2celsius(x);
    let z = fibonacci_loop(6);
    let a = fibonacci_loop(6);

    println!("10 Celsius degrees are {} Farenheit!", x);
    println!("{} Farenheit degrees are {} Celsius!", x, y);
    println!("The 6th Fibonacci number is: {}", z);
    println!("The 6th Fibonacci number is: {}", a);

    println!("\n\nChristmas Song\n");

    song::the_twelve_days_of_christmas();

    println!("\n\nFigures\n");

    let a = Point { x: 6.0, y: 6.0 };
    let b = Point { x: 7.0, y: 7.0 };

    let dist: f32 = point_distance(a, b);
    println!("The distance between {:?} and {:?} is {}", a, b, dist);

    let shape: Shape = Shape::Point { x: (6.0), y: (6.0) };
    let shape_result = shape_area_perimeter(shape);
    println!(
        "SHAPE: {:?}  area = {} perimeter = {}",
        shape, shape_result.0, shape_result.1
    );

    let circ = Circle {
        center_point: a,
        radius: 10.0,
    };
    let circ_area_perimeter = circle_area_perimeter(circ);
    println!(
        "{:?}  area = {}  perimeter = {}",
        circ, circ_area_perimeter.0, circ_area_perimeter.1
    );

    let shape: Shape = Shape::Circle {
        center_point: (a),
        radius: (10.0),
    };
    let shape_result = shape_area_perimeter(shape);
    println!(
        "SHAPE: {:?}  area = {} perimeter = {}",
        shape, shape_result.0, shape_result.1
    );

    let a = Point { x: 0.0, y: 1.0 };
    let b = Point { x: 3.0, y: 4.0 };
    let c = Point { x: 6.0, y: 1.0 };

    let trian = Triangle { a, b, c };
    let trian_area_perimeter = triangle_area_perimeter(trian);
    println!(
        "{:?}  area = {} perimeter = {}",
        trian, trian_area_perimeter.0, trian_area_perimeter.1
    );
    let shape: Shape = Shape::Triangle {
        a: (a),
        b: (b),
        c: (c),
    };
    let shape_result = shape_area_perimeter(shape);
    println!(
        "SHAPE: {:?}  area = {} perimeter = {}",
        shape, shape_result.0, shape_result.1
    );

    let a = Point { x: 1.0, y: 8.0 };
    let b = Point { x: 5.0, y: 8.0 };
    let c = Point { x: 5.0, y: 3.0 };
    // let d = Point { x: 1.0, y: 3.0 };

    let rect = Rectangle { a, b, c };
    let rect_area_perimeter = rectangle_area_perimeter(rect);
    println!(
        "{:?}  area = {} perimeter = {}",
        rect, rect_area_perimeter.0, rect_area_perimeter.1
    );

    let shape: Shape = Shape::Rectangle {
        a: (a),
        b: (b),
        c: (c),
    };
    let shape_result = shape_area_perimeter(shape);
    println!(
        "SHAPE: {:?}  area = {} perimeter = {}",
        shape, shape_result.0, shape_result.1
    );

    println!("\n\nTic Tac Toe\n");

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
    let mut game = TicTacField { board };
    println!("{:?}", game);
    let state = analyze(game);
    println!("{:?} {:?}", game, state);

    let player1 = Player {
        mark: StateBoard::X,
    };
    game = make_move(game, 0, 0, player1).unwrap();
    println!("{:?}", game);

    let state = analyze(game);
    println!("{:?} {:?}", game, state);
}
