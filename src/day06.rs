use core::panic;

fn parse_input(input: &str) -> Vec<(char, Vec<i64>)> {
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    let mut operands: Vec<char> = Vec::new();

    let linecount = input.lines().count();

    for line in input.lines().take(linecount - 1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut nums: Vec<i64> = Vec::new();
        for part in parts {
            match part.parse::<i64>() {
                Ok(num) => nums.push(num),
                Err(_) => panic!("Could not parse input line {part}"),
            }
        }
        numbers.push(nums);
    }

    let op_line = input.lines().nth(linecount - 1).unwrap();
    for ch in op_line.split_whitespace().flat_map(|s| s.chars()) {
        if ch == '+' || ch == '*' {
            operands.push(ch);
        } else {
            panic!("Could not parse operand {ch}");
        }
    }

    // Transpose the numbers with their corresponding operand
    let mut result: Vec<(char, Vec<i64>)> = Vec::new();
    for (i, &op) in operands.iter().enumerate() {
        let mut nums_for_op: Vec<i64> = Vec::new();
        for nums in &numbers {
            nums_for_op.push(nums[i]);
        }
        result.push((op, nums_for_op));
    }
    result
}

fn calculate(op: char, numbers: &Vec<i64>) -> i64 {
    match op {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => panic!("Unsupported operation {op}"),
    }
}

pub fn part1(input: &str) -> i64 {
    let parsed = parse_input(input);
    let mut result: i64 = 0;

    for (op, nums) in parsed {
        result += calculate(op, &nums);
    }

    result
}

pub fn part2(input: &str) -> i32 {
    todo!("Part 2 not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("day06/example1.txt").expect("Example file not found");
        assert_eq!(4277556, part1(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("day06/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day06/example1.txt").expect("Example file not found");
        assert_eq!(3, part2(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day06/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
