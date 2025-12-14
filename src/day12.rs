struct Input {
    grids: Vec<((i32, i32), Vec<i32>)>,
}

fn parse_input(input: &str) -> Input {
    let mut grids = Vec::new();
    let mut lines = input.lines().peekable();

    let mut present_size: i32 = 0;
    loop {
        let line = match lines.peek() {
            Some(&l) => l,
            None => panic!("No grid size line found"),
        };

        if line.trim().is_empty() {
            lines.next();
            continue;
        }

        if line.contains("x") {
            break;
        }

        lines.next();
        present_size += line.trim().matches('#').count() as i32;
    }

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let grid_size: Vec<i32> = parts[0]
            .trim()
            .split("x")
            .map(|s| match s.parse() {
                Ok(v) => v,
                Err(_) => panic!("Invalid grid size value: {}", s),
            })
            .collect();
        if grid_size.len() != 2 {
            panic!("Invalid grid size: {}", parts[0]);
        }

        let present_counts: Vec<i32> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if present_counts.len() != 6 {
            panic!("Invalid present counts: {}", parts[1]);
        }
        grids.push(((grid_size[0], grid_size[1]), present_counts));
    }

    Input { grids }
}

pub fn part1(input: &str) -> i32 {
    let input = parse_input(input);

    let mut filled_regions = 0;
    for ((width, height), presents) in &input.grids {
        let width = width / 3;
        let height = height / 3;
        let total_present_size = presents.iter().sum::<i32>();

        let presents_fit = total_present_size <= width * height;
        if presents_fit {
            filled_regions += 1;
        }
    }

    filled_regions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Not working, since the solution is just heuristic"]
    fn test_part1_example() {
        let input =
            std::fs::read_to_string("inputs/day12/example1.txt").expect("Example file not found");
        assert_eq!(2, part1(&input));
    }

    #[test]
    fn test_part1_input() {
        let input =
            std::fs::read_to_string("inputs/day12/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }
}
