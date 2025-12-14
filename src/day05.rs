fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    let mut end = 0;
    for (offset, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            end = offset;
            break;
        }

        let nums = line
            .split('-')
            .map(|s| match s.parse::<i64>() {
                Ok(n) => n,
                Err(_) => panic!("Unexpected number format"),
            })
            .collect::<Vec<i64>>();
        if nums.len() != 2 {
            panic!("Unexpected range format");
        }
        ranges.push((nums[0], nums[1]));
    }

    for line in input.lines().skip(end + 1) {
        let ingredient = line.parse::<i64>().unwrap();
        ingredients.push(ingredient);
    }
    (ranges, ingredients)
}

fn unique_ranges(ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut sorted = ranges.clone();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    let mut unique = Vec::new();
    let mut current_start = sorted[0].0;
    let mut current_end = sorted[0].1;

    for (start, end) in sorted.iter().skip(1) {
        if *start > current_end + 1 {
            unique.push((current_start, current_end));
            current_start = *start;
            current_end = *end;
        } else if *end > current_end {
            current_end = *end;
        }
    }
    unique.push((current_start, current_end));
    unique
}

pub fn part1(input: &str) -> i32 {
    let (ranges, ingredients) = parse_input(input);
    let mut valid = 0;

    for ingredient in ingredients {
        for (min, max) in &ranges {
            if ingredient >= *min && ingredient <= *max {
                valid += 1;
                break;
            }
        }
    }
    valid
}

pub fn part2(input: &str) -> i64 {
    let (ranges, _ingredients) = parse_input(input);
    let ranges = unique_ranges(&ranges);
    let mut valid: i64 = 0;

    for range in &ranges {
        let (min, max) = range;
        valid += max - min + 1;
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("inputs/day05/example1.txt").expect("Example file not found");
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("inputs/day05/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = std::fs::read_to_string("inputs/day05/example1.txt").expect("Example file not found");
        assert_eq!(14, part2(&input));
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("inputs/day05/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
