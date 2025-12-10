#[derive(Copy, Clone)]
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

pub fn part2(input: &str) -> i32 {
    todo!("Part 2 not yet implemented")
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
    fn test_part1_input() {
        let input = std::fs::read_to_string("day09/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day09/example1.txt").expect("Example file not found");
        assert_eq!(3, part2(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day09/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
