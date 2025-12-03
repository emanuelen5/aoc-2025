struct Digit {
    value: u8,
    offset: i32,
}

fn joltage(line: &str, digits: u8) -> i64 {
    let mut total: i64 = 0;
    let mut start_offset: usize = 0;
    for digit in 0..digits {
        total *= 10;
        let max_digit_i: Digit = max_digit(line, start_offset, digits - digit);
        total += max_digit_i.value as i64;
        start_offset = max_digit_i.offset as usize + 1;
    }

    total
}

fn max_digit(line: &str, start_offset: usize, digits: u8) -> Digit {
    // Go through the rest to find the best second digit
    let mut max_digit: Digit = Digit {
        value: 0,
        offset: -1,
    };
    for (offset, c) in line
        .chars()
        .enumerate()
        .take(line.len() - (digits as usize) + 1)
        .skip(start_offset as usize)
    {
        let digit: u8 = match c.to_digit(10) {
            Some(d) => d as u8,
            None => panic!("Invalid character {c} in input"),
        };

        match digit {
            // We can't find a better value
            9 => {
                max_digit = Digit {
                    value: 9,
                    offset: offset as i32,
                };
                break;
            }
            _ => {
                if digit > max_digit.value {
                    max_digit = Digit {
                        value: digit,
                        offset: offset as i32,
                    };
                }
            }
        }
    }

    if max_digit.offset == -1 {
        panic!("No valid digits found in line after offset {start_offset}");
    }

    max_digit
}

pub fn part1(input: &str) -> i64 {
    let mut total: i64 = 0;
    for line in input.lines() {
        total += joltage(line, 2);
    }
    total
}

pub fn part2(input: &str) -> i64 {
    let mut total: i64 = 0;
    for line in input.lines() {
        total += joltage(line, 12);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("day03/example1.txt").expect("Example file not found");
        assert_eq!(357, part1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day03/example1.txt").expect("Example file not found");
        assert_eq!(3121910778619, part2(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("day03/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day03/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
