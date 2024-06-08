#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use p22::calc::*;
    use p22::figures::*;
    use p22::tictac::*;
    use std::hint::black_box;
    use test::Bencher;

    #[bench]
    fn benchmark_celsius2farenheit(b: &mut Bencher) {
        b.iter(|| black_box(celsius2farenheit(10)));
    }

    #[bench]
    fn benchmark_farenheit2celsius(b: &mut Bencher) {
        b.iter(|| black_box(farenheit2celsius(50)));
    }

    #[bench]
    fn benchmark_fibonacci_loop(b: &mut Bencher) {
        b.iter(|| black_box(fibonacci_loop(6)));
    }

    #[bench]
    fn benchmark_fibonacci_rec(b: &mut Bencher) {
        b.iter(|| black_box(fibonacci_rec(6)));
    }

    #[bench]
    fn benchmark_point_distance(b: &mut Bencher) {
        let pa = Point { x: 6.0, y: 6.0 };
        let pb = Point { x: 7.0, y: 7.0 };

        b.iter(|| black_box(point_distance(pa, pb)));
    }
    #[bench]
    fn benchmark_circle_area_perimeter(b: &mut Bencher) {
        let center_point = Point { x: 6.0, y: 6.0 };
        let radius = 10.0;
        let circ = Circle {
            center_point,
            radius,
        };

        b.iter(|| black_box(circle_area_perimeter(circ)));
    }
    #[bench]
    fn benchmark_triangle_area_perimeter(b: &mut Bencher) {
        let pa = Point { x: 0.0, y: 1.0 };
        let pb = Point { x: 3.0, y: 4.0 };
        let pc = Point { x: 6.0, y: 1.0 };
        let trian = Triangle {
            a: pa,
            b: pb,
            c: pc,
        };

        b.iter(|| black_box(triangle_area_perimeter(trian)));
    }
    #[bench]
    fn benchmark_rectangle_area_perimeter(b: &mut Bencher) {
        let pa = Point { x: 1.0, y: 8.0 };
        let pb = Point { x: 5.0, y: 8.0 };
        let pc = Point { x: 5.0, y: 3.0 };

        let rect = Rectangle {
            a: pa,
            b: pb,
            c: pc,
        };

        b.iter(|| black_box(rectangle_area_perimeter(rect)));
    }
    #[bench]
    fn benchmark_shape_point_area_perimeter(b: &mut Bencher) {
        let pa = Point { x: 6.0, y: 6.0 };
        let shape: Shape = Shape::Point {
            x: (pa.x),
            y: (pa.y),
        };

        b.iter(|| black_box(shape_area_perimeter(shape)));
    }
    #[bench]
    fn benchmark_shape_circle_area_perimeter(b: &mut Bencher) {
        let pa = Point { x: 6.0, y: 6.0 };
        let shape: Shape = Shape::Circle {
            center_point: (pa),
            radius: (10.0),
        };

        b.iter(|| black_box(shape_area_perimeter(shape)));
    }
    #[bench]
    fn benchmark_shape_triangle_area_perimeter(b: &mut Bencher) {
        let pa = Point { x: 0.0, y: 1.0 };
        let pb = Point { x: 3.0, y: 4.0 };
        let pc = Point { x: 6.0, y: 1.0 };
        let shape: Shape = Shape::Triangle {
            a: (pa),
            b: (pb),
            c: (pc),
        };

        b.iter(|| black_box(shape_area_perimeter(shape)));
    }
    #[bench]
    fn benchmark_shape_rectangle_area_perimeter(b: &mut Bencher) {
        let pa = Point { x: 1.0, y: 8.0 };
        let pb = Point { x: 5.0, y: 8.0 };
        let pc = Point { x: 5.0, y: 3.0 };
        let shape: Shape = Shape::Rectangle {
            a: (pa),
            b: (pb),
            c: (pc),
        };

        b.iter(|| black_box(shape_area_perimeter(shape)));
    }

    #[bench]
    fn benchmark_analyze_win_x(b: &mut Bencher) {
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
        b.iter(|| black_box(analyze(game)));
    }
    #[bench]
    fn test_make_move_success_move(b: &mut Bencher) {
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

        b.iter(|| black_box(make_move(game, 0, 0, player1)));
    }
}
