use std::f64::INFINITY;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use crate::{CIRCLE_CENTRE, CIRCLE_RADIUS, HEIGHT, utils, WIDTH};

pub(crate) fn render(canvas: &mut WindowCanvas, frame: i32, sine_points: &mut Vec<Point>, cosine_points: &mut Vec<Point>, tangent_points: &mut Vec<Point>) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    render_graph(canvas);
    render_circle(canvas, CIRCLE_CENTRE.0, CIRCLE_CENTRE.1, CIRCLE_RADIUS, 3);
    let new_points = render_triangle_on_circle(canvas, frame);
    render_sine_wave(canvas, sine_points, new_points.0.y);
    render_cosine_wave(canvas, cosine_points, new_points.0.x);
    render_tangent_wave(canvas, tangent_points, new_points.1.y);
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
    let new_point = Point::new(CIRCLE_CENTRE.0 + CIRCLE_RADIUS, new_y);
    points.push(new_point);
    if points.len() > 1000 {
        points.remove(0);
    }
    for i in 0..points.len() - 1 {
        if points[i].x == 0 && points[i].y == 0 { continue; }
        canvas.draw_line(points[i], points[i + 1]).expect("Failed to draw line");
    }
    for i in 0..points.len() {
        if points[i].x == 0 && points[i].y == 0 { continue; }
        points[i].x += 5;
    }
}

fn render_cosine_wave(canvas: &mut WindowCanvas, points: &mut Vec<Point>, new_x: i32) {
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    let new_point = Point::new(new_x, CIRCLE_CENTRE.1 - CIRCLE_RADIUS);
    points.push(new_point);
    if points.len() > 1000 {
        points.remove(0);
    }
    for i in 0..points.len() - 1 {
        if points[i].x == 0 && points[i].y == 0 { continue; }
        canvas.draw_line(points[i], points[i + 1]).expect("Failed to draw line");
    }
    for i in 0..points.len() {
        if points[i].x == 0 && points[i].y == 0 { continue; }
        points[i].y -= 5;
    }
}

fn render_tangent_wave(canvas: &mut WindowCanvas, points: &mut Vec<Point>, new_y: i32) {
    canvas.set_draw_color(Color::RGB(150, 0, 150));
    canvas.set_draw_color(Color::RGB(0, 150, 150));
    let new_point = Point::new(CIRCLE_CENTRE.0 + CIRCLE_RADIUS, new_y);
    if points.len() > 1000 {
        points.remove(0);
    }

    points.push(new_point);
    for i in 0..points.len() - 1 {
        if points[i].x == 0 && points[i].y == 0 {
            continue;
        }
        if (points[i].y - points[i + 1].y).abs() > 1000 {
            continue;
        }
        canvas.draw_line(points[i], points[i + 1]).expect("Failed to draw line");
    }
    for i in 0..points.len() {
        if points[i].x == 0 && points[i].y == 0 {
            continue;
        }
    }
    for i in 0..points.len() {
        if points[i].x == 0 && points[i].y == 0 {
            continue;
        }
        points[i].x += 5;
    }
}

fn render_graph(canvas: &mut WindowCanvas) {
    for i in -1..16 {
        canvas.set_draw_color(Color::RGB(150, 150, 150));
        canvas.draw_line(Point::new(CIRCLE_CENTRE.0 + i * 100, 0), Point::new(CIRCLE_CENTRE.0 + i * 100, HEIGHT)).expect("Failed to draw line");
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        for j in 1..10 {
            canvas.draw_line(Point::new(CIRCLE_CENTRE.0 + i * 100 - j * 10, 0), Point::new(CIRCLE_CENTRE.0 + i * 100 - j * 10, HEIGHT)).expect("Failed to draw line");
        }
    }
    for i in -2..8 {
        canvas.set_draw_color(Color::RGB(150, 150, 150));
        canvas.draw_line(Point::new(0, CIRCLE_CENTRE.1 - i * 100), Point::new(WIDTH, CIRCLE_CENTRE.1 - i * 100)).expect("Failed to draw line");
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        for j in 1..10 {
            canvas.draw_line(Point::new(0, CIRCLE_CENTRE.1 - i * 100 - j * 10), Point::new(WIDTH, CIRCLE_CENTRE.1 - i * 100 - j * 10)).expect("Failed to draw line");
        }
    }
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_line(Point::new(0, CIRCLE_CENTRE.1), Point::new(WIDTH, CIRCLE_CENTRE.1)).expect("Failed to draw line");
    canvas.draw_line(Point::new(CIRCLE_CENTRE.0, 0), Point::new(CIRCLE_CENTRE.0, HEIGHT)).expect("Failed to draw line");
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

fn render_triangle_on_circle(canvas: &mut WindowCanvas, frame: i32) -> (Point, Point) {
    let center = Point::new(CIRCLE_CENTRE.0, CIRCLE_CENTRE.1);
    let changing_point = Point::new(CIRCLE_CENTRE.0 + CIRCLE_RADIUS, CIRCLE_CENTRE.1);

    let theta = -(frame as f64) / 15.0;
    let (p1, p2) = utils::rotate_line_around_point((center, changing_point), center, theta);
    render_right_angled_triangle(canvas, p2, p1);
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    render_full_circle_pixels(canvas, p2.x, p2.y, 3);

    // sin
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    render_full_circle_pixels(canvas, CIRCLE_CENTRE.0 + CIRCLE_RADIUS, p2.y, 3);
    canvas.draw_line(p2, Point::new(CIRCLE_CENTRE.0 + CIRCLE_RADIUS, p2.y)).expect("Failed to draw line");
    // cos
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    render_full_circle_pixels(canvas, p2.x, CIRCLE_CENTRE.1 - CIRCLE_RADIUS, 3);
    canvas.draw_line(p2, Point::new(p2.x, CIRCLE_CENTRE.1 - CIRCLE_RADIUS)).expect("Failed to draw line");
    // tan
    canvas.set_draw_color(Color::RGB(0, 150, 150));
    let gradient = (p2.y - CIRCLE_CENTRE.1) as f64 / (p2.x - CIRCLE_CENTRE.0) as f64;
    let mut p3 = utils::get_lines_intercept(p2, gradient, Point::new(CIRCLE_CENTRE.0 + 100, CIRCLE_CENTRE.1), INFINITY);
    if gradient > 1000.0 || gradient < -1000.0 {
        p3 = Point::new(CIRCLE_CENTRE.0 + 100, CIRCLE_CENTRE.1);
    }
    render_full_circle_pixels(canvas, p3.x, p3.y, 3);
    canvas.draw_line(p2, p3).expect("Failed to draw line");

    (p2, p3)
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

fn render_thick_line(canvas: &mut WindowCanvas, p1: Point, p2: Point, thickness: i32) {
    // Draw multiple lines to create a thick line
    let gradient = (p2.y - p1.y) as f64 / (p2.x - p1.x) as f64;
    let mut sign = 1;
    for i in 0..thickness {
        sign = -sign;
        let offset = if i % 2 == 0 { i / 2 } else { i };
        if gradient.is_infinite() {
            canvas.draw_line(Point::new(p1.x + sign * offset, p1.y), Point::new(p2.x + sign * offset, p2.y)).expect("Failed to draw line");
            continue;
        }
        let perp_gradient = -1.0 / gradient;
        if perp_gradient.is_infinite() {
            canvas.draw_line(Point::new(p1.x, p1.y + sign * offset), Point::new(p2.x, p2.y + sign * offset)).expect("Failed to draw line");
            continue;
        }
        let (p1, p2) = (Point::new(p1.x + sign * offset, (perp_gradient * (p1.x - p2.x) as f64 + p2.y as f64) as i32), Point::new(p2.x + sign * offset, (perp_gradient * (p2.x - p1.x) as f64 + p1.y as f64) as i32));
        canvas.draw_line(p1, p2).expect("Failed to draw line");
    }
}