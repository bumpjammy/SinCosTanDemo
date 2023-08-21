use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub(crate) fn render(canvas: &mut WindowCanvas, frame: i32, sine_points: &mut Vec<Point>, cosine_points: &mut Vec<Point>, tangent_points: &mut Vec<Point>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    render_graph(canvas);
    render_circle(canvas, 200, 600, 100, 3);
    let new_point = render_triangle_on_circle(canvas, frame);
    render_sine_wave(canvas, sine_points, new_point.y);
    render_cosine_wave(canvas, cosine_points, new_point.x);
    canvas.present();
}

fn render_circle(canvas: &mut WindowCanvas, center_x: i32, center_y: i32, radius: i32, thickness: i32) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    render_full_circle_pixels(canvas, center_x, center_y, radius);
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    render_full_circle_pixels(canvas, center_x, center_y, radius - thickness);
}

fn render_sine_wave(canvas: &mut WindowCanvas, points: &mut Vec<Point>, new_y: i32) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let new_point = Point::new(330, new_y);
    points.push(new_point);
    for i in 0..points.len() - 1 {
        canvas.draw_line(points[i], points[i + 1]).expect("Failed to draw line");
    }
    for i in 0..points.len() {
        points[i].x += 5;
    }
}

fn render_cosine_wave(canvas: &mut WindowCanvas, points: &mut Vec<Point>, new_x: i32) {
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    let new_point = Point::new(new_x, 470);
    points.push(new_point);
    for i in 0..points.len() - 1 {
        canvas.draw_line(points[i], points[i + 1]).expect("Failed to draw line");
    }
    for i in 0..points.len() {
        points[i].y -= 5;
    }
}

fn render_graph(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for i in -1..16 {
        canvas.set_draw_color(Color::RGB(150, 150, 150));
        canvas.draw_line(Point::new(200 + i * 100, 0), Point::new(200 + i * 100, 800)).expect("Failed to draw line");
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        for j in 1..10 {
            canvas.draw_line(Point::new(200 + i * 100 - j * 10, 0), Point::new(200 + i * 100 - j * 10, 800)).expect("Failed to draw line");
        }
    }
    for i in -2..8 {
        canvas.set_draw_color(Color::RGB(150, 150, 150));
        canvas.draw_line(Point::new(0, 600 - i * 100), Point::new(1600, 600 - i * 100)).expect("Failed to draw line");
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        for j in 1..10 {
            canvas.draw_line(Point::new(0, 600 - i * 100 - j * 10), Point::new(1600, 600 - i * 100 - j * 10)).expect("Failed to draw line");
        }
    }
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_line(Point::new(0, 600), Point::new(1600, 600)).expect("Failed to draw line");
    canvas.draw_line(Point::new(200, 0), Point::new(200, 800)).expect("Failed to draw line");
}

fn render_full_circle_pixels(canvas: &mut WindowCanvas, center_x: i32, center_y: i32, radius: i32) {
    let mut x = radius;
    let mut y = 0;
    let mut x_change = 1 - (radius << 1);
    let mut y_change = 0;
    let mut radius_error = 1 - x;

    while x >= y {
        for i in center_x - x..center_x + x {
            canvas.draw_point(Point::new(i, center_y + y)).expect("Failed to draw point");
            canvas.draw_point(Point::new(i, center_y - y)).expect("Failed to draw point");
        }
        for i in center_x - y..center_x + y {
            canvas.draw_point(Point::new(i, center_y + x)).expect("Failed to draw point");
            canvas.draw_point(Point::new(i, center_y - x)).expect("Failed to draw point");
        }

        y += 1;
        radius_error += y_change;
        y_change += 2;
        if ((radius_error << 1) + x_change) > 0 {
            x -= 1;
            radius_error += x_change;
            x_change += 2;
        }
    }
}

fn render_triangle_on_circle(canvas: &mut WindowCanvas, frame: i32) -> Point {
    let center = Point::new(200, 600);
    let changing_point = Point::new(300, 600);

    let theta = -(frame as f64) / 15.0;
    let (p1, p2) = rotate_line_around_point((center, changing_point), center, theta);
    render_right_angled_triangle(canvas, p2, p1);
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    render_full_circle_pixels(canvas, p2.x, p2.y, 3);

    // sin
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    render_full_circle_pixels(canvas, 330, p2.y, 3);
    canvas.draw_line(p2, Point::new(330, p2.y)).expect("Failed to draw line");
    // cos
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    render_full_circle_pixels(canvas, p2.x, 470, 3);
    canvas.draw_line(p2, Point::new(p2.x, 470)).expect("Failed to draw line");

    p2
}


fn rotate_line_around_point(line: (Point, Point), point: Point, theta: f64) -> (Point, Point) {
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

fn render_right_angled_triangle(canvas: &mut WindowCanvas, p1: Point, p2: Point) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_line(p1, p2).expect("Failed to draw line");
    canvas.draw_line(p2, Point::new(p1.x, p2.y)).expect("Failed to draw line");
    canvas.draw_line(Point::new(p1.x, p2.y), p1).expect("Failed to draw line");
}

fn render_triangle(canvas: &mut WindowCanvas, p1: Point, p2 : Point, p3: Point) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_line(p1, p2).expect("Failed to draw line");
    canvas.draw_line(p2, p3).expect("Failed to draw line");
    canvas.draw_line(p3, p1).expect("Failed to draw line");
}