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
        does
    }) {
        return true;
    }

    false
}

pub fn part2(input: &str) -> i64 {
    let points = parse_input(input);
    let mut largest_area = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = rectangle_area(points[i], points[j]);
            if area <= largest_area {
                continue;
            }

            if has_white_inside(points[i], points[j], &points) {
                continue;
            }

            largest_area = area;
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
