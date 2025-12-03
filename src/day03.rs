struct Digit {
    value: u8,
    offset: i32,
}

fn joltage(line: &str) -> i32 {
    // All but the last character
    let mut max_digit0: Digit = Digit {
        value: 0,
        offset: -1,
    };
    for (offset, c) in line.chars().take(line.len() - 1).enumerate() {
        let digit: u8 = match c.to_digit(10) {
            Some(d) => d as u8,
            None => panic!("Invalid character {c} in input"),
        };

        match digit {
            // We can't find a better value
            9 => {
                max_digit0 = Digit {
                    value: 9,
                    offset: offset as i32,
                };
                break;
            }
            _ => {
                if digit > max_digit0.value {
                    max_digit0 = Digit {
                        value: digit,
                        offset: offset as i32,
                    };
                }
            }
        }
    }

    if max_digit0.offset == -1 {
        panic!("No digits found in line");
    }

    // Go through the rest to find the best second digit
    let mut max_digit1: Digit = Digit {
        value: 0,
        offset: -1,
    };
    for (offset, c) in line
        .chars()
        .enumerate()
        .skip((max_digit0.offset + 1) as usize)
    {
        let digit: u8 = match c.to_digit(10) {
            Some(d) => d as u8,
            None => panic!("Invalid character {c} in input"),
        };

        match digit {
            // We can't find a better value
            9 => {
                max_digit1 = Digit {
                    value: 9,
                    offset: offset as i32,
                };
                break;
            }
            _ => {
                if digit > max_digit1.value {
                    max_digit1 = Digit {
                        value: digit,
                        offset: offset as i32,
                    };
                }
            }
        }
    }

    if max_digit1.offset == -1 {
        panic!("No second digit found in line");
    }

    (max_digit0.value as i32) * 10 + (max_digit1.value as i32)
}

pub fn part1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        total += joltage(line);
    }
    total
}

pub fn part2(input: &str) -> i32 {
    todo!("Part 2 not yet implemented")
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
    #[ignore]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day03/example1.txt").expect("Example file not found");
        assert_eq!(3, part2(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("day03/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day03/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
