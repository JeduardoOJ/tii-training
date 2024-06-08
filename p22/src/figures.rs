#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Circle {
    pub center_point: Point,
    pub radius: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
    pub c: Point, // a third point is necessary if the rectangle is rotated
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Point { x: f32, y: f32 },
    Circle { center_point: Point, radius: f32 },
    Triangle { a: Point, b: Point, c: Point },
    Rectangle { a: Point, b: Point, c: Point },
}

/// Compute the distance between two points
/// ```
/// use p22::figures::*;
/// let a = Point { x: 6.0, y: 6.0 };
/// let b = Point { x: 7.0, y: 7.0 };
/// assert_eq!(point_distance(a, b), point_distance(b, a));
/// assert_ne!(point_distance(a, a), point_distance(b, a));
/// ```
pub fn point_distance(a: Point, b: Point) -> f32 {
    let x = b.x - a.x;
    let y = b.y - a.y;
    let d = (x * x) + (y * y);
    d.sqrt()
}
/// Compute the area and perimeter of a circle
/// ```
/// use p22::figures::*;
/// let center_point = Point { x: 6.0, y: 6.0 };
/// let radius = 10.0;
/// let circ = Circle {
///     center_point,
///     radius,
/// };
// assert_eq!(circle_area_perimeter(circ), (314.15927, 62.831856));
/// ```
pub fn circle_area_perimeter(circle: Circle) -> (f32, f32) {
    let pi_radius = std::f32::consts::PI * circle.radius;
    (circle.radius * pi_radius, 2.0 * pi_radius)
}
/// Compute the area and perimeter of a triangle
/// ```
/// use p22::figures::*;
/// let a = Point { x: 0.0, y: 1.0 };
/// let b = Point { x: 3.0, y: 4.0 };
/// let c = Point { x: 6.0, y: 1.0 };
/// let trian = Triangle { a, b, c };
/// assert_eq!(triangle_area_perimeter(trian), (8.999999, 14.485281));
/// ```
pub fn triangle_area_perimeter(triangle: Triangle) -> (f32, f32) {
    let x = point_distance(triangle.a, triangle.b);
    let y = point_distance(triangle.b, triangle.c);
    let z = point_distance(triangle.c, triangle.a);
    let perimeter = x + y + z;
    let s = perimeter / 2.0;
    ((s * (s - x) * (s - y) * (s - z)).sqrt(), perimeter)
}
/// Compute the area and perimeter of a rectangle
/// ```
/// use p22::figures::*;
/// let a = Point { x: 1.0, y: 8.0 };
/// let b = Point { x: 5.0, y: 8.0 };
/// let c = Point { x: 5.0, y: 3.0 };
/// let rectangle = Rectangle { a, b, c };
/// assert_eq!(rectangle_area_perimeter(rectangle), (20.0, 18.0));
/// ```
pub fn rectangle_area_perimeter(rectangle: Rectangle) -> (f32, f32) {
    let x = point_distance(rectangle.a, rectangle.b);
    let y = point_distance(rectangle.b, rectangle.c);
    (x * y, 2.0 * x + 2.0 * y)
}
/// Compute the area and perimeter of a shape (point, circle, triangle, rectangle)
/// ```
/// use p22::figures::*;
/// let a = Point { x: 1.0, y: 8.0 };
/// let shape: Shape = Shape::Circle {
///     center_point: (a),
///     radius: (10.0),
/// };
/// assert_eq!(shape_area_perimeter(shape), (314.15927, 62.831856));
/// ```
pub fn shape_area_perimeter(shape: Shape) -> (f32, f32) {
    match shape {
        Shape::Point { x: _, y: _ } => (0.0, 0.0),
        Shape::Circle {
            center_point,
            radius,
        } => circle_area_perimeter(Circle {
            center_point,
            radius,
        }),
        Shape::Triangle { a, b, c } => triangle_area_perimeter(Triangle { a, b, c }),
        Shape::Rectangle { a, b, c } => rectangle_area_perimeter(Rectangle { a, b, c }),
    }
}

#[cfg(test)]
mod tests {
    use crate::figures::*;
    #[test]
    fn test_point() {
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
    #[test]
    fn test_shape() {
        let a = Point { x: 6.0, y: 6.0 };
        let shape: Shape = Shape::Point { x: (a.x), y: (a.y) };
        assert_eq!(shape_area_perimeter(shape), (0.0, 0.0));

        let shape: Shape = Shape::Circle {
            center_point: (a),
            radius: (10.0),
        };
        assert_eq!(shape_area_perimeter(shape), (314.15927, 62.831856));

        let a = Point { x: 0.0, y: 1.0 };
        let b = Point { x: 3.0, y: 4.0 };
        let c = Point { x: 6.0, y: 1.0 };

        let shape: Shape = Shape::Triangle {
            a: (a),
            b: (b),
            c: (c),
        };
        assert_eq!(shape_area_perimeter(shape), (8.999999, 14.485281));

        let a = Point { x: 1.0, y: 8.0 };
        let b = Point { x: 5.0, y: 8.0 };
        let c = Point { x: 5.0, y: 3.0 };

        let shape: Shape = Shape::Rectangle {
            a: (a),
            b: (b),
            c: (c),
        };
        assert_eq!(shape_area_perimeter(shape), (20.0, 18.0));
    }
}
