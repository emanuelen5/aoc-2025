#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|part| part.parse().expect("Invalid number"))
                .collect();
            Point {
                x: parts[0],
                y: parts[1],
            }
        })
        .collect()
}

fn rectangle_area(p1: Point, p2: Point) -> i64 {
    let width = (p2.x - p1.x).abs() + 1;
    let height = (p2.y - p1.y).abs() + 1;
    width * height
}

pub fn part1(input: &str) -> i64 {
    let points = parse_input(input);
    let mut largest_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = rectangle_area(points[i], points[j]);
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    largest_area
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

fn line_crosses_borders(l: &Line, rectangle: &(Point, Point)) -> bool {
    let rmin_x = rectangle.0.x.min(rectangle.1.x);
    let rmax_x = rectangle.0.x.max(rectangle.1.x);
    let rmin_y = rectangle.0.y.min(rectangle.1.y);
    let rmax_y = rectangle.0.y.max(rectangle.1.y);
    let pmin_x = l.p1.x.min(l.p2.x);
    let pmax_x = l.p1.x.max(l.p2.x);
    let pmin_y = l.p1.y.min(l.p2.y);
    let pmax_y = l.p1.y.max(l.p2.y);

    let is_horizontal = l.p1.y == l.p2.y;
    let y = l.p1.y;
    let is_inside = rmin_y < y && y < rmax_y;
    if is_horizontal && is_inside {
        if pmin_x <= rmin_x && rmin_x < pmax_x {
            return true;
        }
        if pmin_x < rmax_x && rmax_x <= pmax_x {
            return true;
        }
    }

    let is_vertical = l.p1.x == l.p2.x;
    let x = l.p1.x;
    let is_inside = rmin_x < x && x < rmax_x;
    if is_vertical && is_inside {
        if pmin_y <= rmin_y && rmin_y < pmax_y {
            return true;
        }
        if pmin_y < rmax_y && rmax_y <= pmax_y {
            return true;
        }
    }
    false
}

fn has_white_inside(p1: Point, p2: Point, points: &[Point]) -> bool {
    let mut points: Vec<Point> = points.to_vec();
    points.push(points[0]); // So we get back to the start

    // Lines are two consecutive points
    let lines: Vec<Line> = points
        .windows(2)
        .map(|w| Line { p1: w[0], p2: w[1] })
        .collect();

    let rectangle = (p1, p2);
    if lines.iter().any(|l| {
        let does = line_crosses_borders(&l, &rectangle);
        println!("Line {:?} crosses borders: {}", l, does);
        does
    }) {
        return true;
    }

    false
}

fn is_on_line(p: Point, p1: Point, p2: Point) -> bool {
    // Check if point p is on the line segment between p1 and p2
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    if p.x < min_x || p.x > max_x || p.y < min_y || p.y > max_y {
        return false;
    }

    // Horizontal line
    if p1.y == p2.y && p.y == p1.y {
        return true;
    }

    // Vertical line
    if p1.x == p2.x && p.x == p1.x {
        return true;
    }

    false
}

fn draw_grid(points: &[Point], rect_p1: Point, rect_p2: Point, max_rect: (Point, Point)) {
    // Find grid bounds
    let min_x = points.iter().map(|p| p.x).min().unwrap_or(0);
    let max_x = points.iter().map(|p| p.x).max().unwrap_or(0);
    let min_y = points.iter().map(|p| p.y).min().unwrap_or(0);
    let max_y = points.iter().map(|p| p.y).max().unwrap_or(0);

    // ANSI color codes
    const RESET: &str = "\x1b[0m";
    const GRAY: &str = "\x1b[90m";
    const WHITE: &str = "\x1b[97m";
    const RED: &str = "\x1b[91m";
    const YELLOW: &str = "\x1b[93m";

    // Get rectangle bounds
    let rect_min_x = rect_p1.x.min(rect_p2.x);
    let rect_max_x = rect_p1.x.max(rect_p2.x);
    let rect_min_y = rect_p1.y.min(rect_p2.y);
    let rect_max_y = rect_p1.y.max(rect_p2.y);

    // Get max rectangle bounds if it exists
    let max_rect_bounds = {
        let (p1, p2) = max_rect;
        (
            p1.x.min(p2.x),
            p1.x.max(p2.x),
            p1.y.min(p2.y),
            p1.y.max(p2.y),
        )
    };

    // Create lines between consecutive points (including wrap-around)
    let mut extended_points = points.to_vec();
    extended_points.push(points[0]);

    println!("\n");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let current = Point { x, y };
            let is_point = points.iter().any(|p| p.x == x && p.y == y);
            let is_on_any_line = extended_points
                .windows(2)
                .any(|w| is_on_line(current, w[0], w[1]));
            let is_in_rect =
                x >= rect_min_x && x <= rect_max_x && y >= rect_min_y && y <= rect_max_y;
            let is_rect_corner = is_in_rect
                && (current.x == rect_p1.x && current.y == rect_p1.y
                    || current.x == rect_p2.x && current.y == rect_p2.y);
            let is_in_max_rect = {
                let (min_x, max_x, min_y, max_y) = max_rect_bounds;
                x >= min_x && x <= max_x && y >= min_y && y <= max_y
            };
            let is_max_rect_corner = if let (p1, p2) = max_rect {
                is_in_max_rect
                    && (current.x == p1.x && current.y == p1.y
                        || current.x == p2.x && current.y == p2.y)
            } else {
                false
            };

            if is_point {
                if is_max_rect_corner {
                    print!("{RED}O{RESET}");
                } else if is_rect_corner {
                    print!("{YELLOW}#{RESET}");
                } else {
                    print!("{WHITE}#{RESET}");
                }
            } else if is_on_any_line {
                print!("{GRAY}X{RESET}");
            } else {
                print!("{GRAY}.{RESET}");
            }
        }
        println!();
    }
}

pub fn part2(input: &str) -> i64 {
    let points = parse_input(input);
    let mut largest_area = 0;
    let mut max_rect: (Point, Point) = (Point { x: 0, y: 0 }, Point { x: 0, y: 0 });

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = rectangle_area(points[i], points[j]);
            if area <= largest_area {
                continue;
            }

            // Draw the grid with current rectangle and max rectangle
            draw_grid(&points, points[i], points[j], max_rect);
            println!(
                "Current area: {} between points ({}, {}) and ({}, {})",
                largest_area, max_rect.0.x, max_rect.0.y, max_rect.1.x, max_rect.1.y
            );
            println!(
                "Testing points ({}, {}) and ({}, {})",
                points[i].x, points[i].y, points[j].x, points[j].y
            );

            if has_white_inside(points[i], points[j], &points) {
                println!("Discarded - has white inside");
                std::thread::sleep(std::time::Duration::from_millis(500));
                continue;
            }

            println!("New largest area: {}", area);
            largest_area = area;
            max_rect = (points[i], points[j]);
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
    largest_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("day09/example1.txt").expect("Example file not found");
        assert_eq!(50, part1(&input));
    }

    #[test]
    fn test_rectangle_area() {
        assert_eq!(
            24,
            rectangle_area(Point { x: 2, y: 5 }, Point { x: 9, y: 7 })
        );
        assert_eq!(
            35,
            rectangle_area(Point { x: 7, y: 1 }, Point { x: 11, y: 7 })
        );
    }

    #[test]
    fn test_crossing_line() {
        assert_eq!(
            true,
            line_crosses_borders(
                &Line {
                    p1: Point { x: 9, y: 5 },
                    p2: Point { x: 2, y: 5 },
                },
                &(Point { x: 7, y: 1 }, Point { x: 11, y: 7 })
            )
        );
    }

    #[test]
    fn test_crossing_line_outside() {
        assert_eq!(
            true,
            line_crosses_borders(
                &Line {
                    p1: Point { x: 0, y: 1 },
                    p2: Point { x: 5, y: 1 },
                },
                &(Point { x: 1, y: 0 }, Point { x: 3, y: 2 })
            )
        );
    }

    #[test]
    fn test_crossing_line_on_border() {
        assert_eq!(
            true,
            line_crosses_borders(
                &Line {
                    p1: Point { x: 1, y: 1 },
                    p2: Point { x: 3, y: 1 },
                },
                &(Point { x: 1, y: 0 }, Point { x: 3, y: 2 })
            )
        );
    }

    #[test]
    fn test_crossing_line_on_border_failing() {
        assert_eq!(
            true,
            line_crosses_borders(
                &Line {
                    p1: Point { x: 7, y: 1 },
                    p2: Point { x: 7, y: 3 },
                },
                &(Point { x: 11, y: 1 }, Point { x: 2, y: 3 })
            )
        );
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("day09/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day09/example1.txt").expect("Example file not found");
        assert_eq!(24, part2(&input));
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day09/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
