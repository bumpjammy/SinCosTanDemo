use sdl2::rect::Point;

pub(crate) fn rotate_line_around_point(line: (Point, Point), point: Point, theta: f64) -> (Point, Point) {
    let (p1, p2) = line;
    let (x1, y1) = (p1.x as f64, p1.y as f64);
    let (x2, y2) = (p2.x as f64, p2.y as f64);
    let (x, y) = (point.x as f64, point.y as f64);
    let (cos, sin) = (theta.cos(), theta.sin());
    let (x1, y1) = (x1 - x, y1 - y);
    let (x2, y2) = (x2 - x, y2 - y);
    let (x1, y1) = (x1 * cos - y1 * sin, x1 * sin + y1 * cos);
    let (x2, y2) = (x2 * cos - y2 * sin, x2 * sin + y2 * cos);
    let (x1, y1) = (x1 + x, y1 + y);
    let (x2, y2) = (x2 + x, y2 + y);
    (Point::new(x1 as i32, y1 as i32), Point::new(x2 as i32, y2 as i32))
}

pub(crate) fn get_lines_intercept(p1: Point, gradient1: f64, p2: Point, gradient2: f64) -> Point {
    if gradient2.is_infinite() {
        return Point::new(p2.x, (gradient1 * (p2.x - p1.x) as f64 + p1.y as f64) as i32);
    }
    if gradient1.is_infinite() {
        return Point::new(p1.x, (gradient2 * (p1.x - p2.x) as f64 + p2.y as f64) as i32);
    }
    let x = ((gradient1 * p1.x as f64 - gradient2 * p2.x as f64) + (p2.y - p1.y) as f64) / (gradient1 - gradient2);
    let y = gradient1 * (x - p1.x as f64) + p1.y as f64;
    Point::new(x as i32, y as i32)
}