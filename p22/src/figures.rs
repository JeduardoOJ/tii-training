#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Circle {
    pub center_point: Point,
    pub radius: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
}

#[derive(Debug)]
pub enum Shape {
    Point,
    Circle,
    Triangle,
    Rectangle,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub fn distance(&self, p: Point) -> u32 {
        ((p.x - self.x) * (p.x - self.x)) + ((p.y - self.y) * (p.y - self.y))
    }
}

impl Circle {
    pub fn new(center_point: Point, radius: u32) -> Self {
        Self {
            center_point,
            radius,
        }
    }
    pub fn area(&self) -> u32 {
        self.center_point.x * self.radius
    }
    pub fn perimeter(&self) -> u32 {
        self.center_point.x * self.radius
    }
}
impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }
    pub fn area(&self) -> u32 {
        self.a.x * self.b.x * self.c.x
    }
    pub fn perimeter(&self) -> u32 {
        self.a.x * self.b.x * self.c.x
    }
}

impl Rectangle {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
    pub fn area(&self) -> u32 {
        self.a.x * self.b.x
    }
    pub fn perimeter(&self) -> u32 {
        self.a.x * self.b.x
    }
}
