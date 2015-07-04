#[derive(Copy, Clone)]
pub struct Point2D {
    x: f64,
    y: f64
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x: x, y: y }
    }

    pub fn ccw(a: Point2D, b: Point2D, c: Point2D) -> i32 {
        let area2 = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
        match area2 {
            _ if area2 < 0f64 => -1,
            _ if area2 > 0f64 =>  1,
            _                 =>  0
        }
    }
}
