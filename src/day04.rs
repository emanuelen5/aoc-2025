fn parse_array(input: &str) -> Vec<Vec<bool>> {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let mut padded_array = vec![vec![false; cols + 2]; rows + 2];

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            padded_array[i + 1][j + 1] = match c {
                '@' => true,
                '.' => false,
                _ => panic!("Unexpected character in input"),
            }
        }
    }

    padded_array
}

fn _print_array(arr: &Vec<Vec<bool>>) {
    for row in arr {
        for &cell in row {
            let c = if cell { '@' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
}

fn neighbors(arr: &Vec<Vec<bool>>, row: usize, col: usize) -> u32 {
    let mut count = 0;
    for i in (row - 1)..=(row + 1) {
        for j in (col - 1)..=(col + 1) {
            if i == row && j == col {
                continue;
            }
            if arr[i][j] {
                count += 1;
            }
        }
    }
    count
}

fn remove_accessible_rolls(arr: &mut Vec<Vec<bool>>) -> u32 {
    let mut to_remove: Vec<(usize, usize)> = Vec::new();
    let mut total_with_less_than_4_neighbors = 0;
    for i in 1..(arr.len() - 1) {
        for j in 1..(arr[0].len() - 1) {
            if !arr[i][j] {
                continue;
            }

            let n = neighbors(&arr, i, j);
            if n < 4 {
                total_with_less_than_4_neighbors += 1;
                to_remove.push((i, j));
            }
        }
    }
    for (i, j) in to_remove {
        arr[i][j] = false;
    }
    total_with_less_than_4_neighbors
}

pub fn part1(input: &str) -> u32 {
    let mut arr = parse_array(input);
    remove_accessible_rolls(&mut arr)
}

pub fn part2(input: &str) -> u32 {
    let mut arr = parse_array(input);
    let mut removed = 0;
    loop {
        let n = remove_accessible_rolls(&mut arr);
        if n == 0 {
            break;
        }
        removed += n;
    }
    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("inputs/day04/example1.txt").expect("Example file not found");
        assert_eq!(13, part1(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("inputs/day04/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = std::fs::read_to_string("inputs/day04/example1.txt").expect("Example file not found");
        assert_eq!(43, part2(&input));
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("inputs/day04/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
