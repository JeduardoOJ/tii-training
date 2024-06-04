use figures::*;

pub mod calc;
pub mod figures;
pub mod song;

fn main() {
    let x = calc::celsius2farenheit(10i32);
    let y = calc::farenheit2celsius(x);
    let z = calc::fibonacci_loop(6);
    let a = calc::fibonacci_loop(6);

    println!("Hello, world! {x} {y} {z} {a}");

    let a: Point = Point::new(6, 6);
    let b: Point = Point::new(7, 7);
    let c: Point = Point::new(8, 8);

    let dist: u32 = Point::distance(&a, b);
    println!("{:?} distance = {}", a, dist);

    let circ: Circle = Circle::new(a, 10);
    let area_circ: u32 = Circle::area(&circ);
    let peri_circ: u32 = Circle::perimeter(&circ);
    println!(
        "{:?}  area = {}  perimeter = {}",
        circ, area_circ, peri_circ
    );

    let trian: Triangle = Triangle::new(a, b, c);
    let area_trian: u32 = Triangle::area(&trian);
    let peri_trian: u32 = Triangle::perimeter(&trian);
    println!(
        "{:?}  area = {} perimeter = {}",
        trian, area_trian, peri_trian
    );

    let rect: Rectangle = Rectangle::new(a, b);
    let area_rect: u32 = Rectangle::area(&rect);
    let peri_rect: u32 = Rectangle::perimeter(&rect);
    println!("{:?}  area = {} perimeter = {}", rect, area_rect, peri_rect);
}
