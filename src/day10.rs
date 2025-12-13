use core::panic;

use z3::Optimize;
use z3::ast;

struct Config {
    lights_on: Vec<bool>,
    toggles_indices: Vec<Vec<usize>>,
    joltages: Vec<i32>,
}

fn parse_line(line: &str) -> Config {
    let parts: Vec<&str> = line.split(' ').collect();

    let lights_on_str = parts[0].trim_matches(&['[', ']'][..]).chars();
    let lights_on: Vec<bool> = lights_on_str.map(|c| c == '#').collect();

    let mut toggles_indices = Vec::new();
    let toggles_parts = &parts[1..parts.len() - 1];
    for part in toggles_parts {
        let indices_str = part.trim_matches(&['(', ')'][..]);
        let indices: Vec<usize> = indices_str
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        toggles_indices.push(indices);
    }

    let joltages: Vec<i32> = parts[parts.len() - 1]
        .trim_matches(&['{', '}'][..])
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    if lights_on.len() != joltages.len() {
        panic!("Number of lights does not match number of joltages");
    }

    Config {
        lights_on,
        toggles_indices,
        joltages,
    }
}

fn parse_input(input: &str) -> Vec<Config> {
    let mut configs = Vec::new();

    for line in input.lines() {
        configs.push(parse_line(line));
    }
    configs
}

fn solve_subproblem(config: Config) -> i32 {
    let optimizer = Optimize::new();
    let mut light_togglers: Vec<Vec<&ast::Int>> = vec![Vec::new(); config.lights_on.len()];

    let mut buttons_pressed: Vec<ast::Int> = Vec::new();
    for i in 0..config.toggles_indices.len() {
        let button_press_count = ast::Int::new_const(format!("button_{}", i));
        optimizer.assert(&button_press_count.ge(0));
        buttons_pressed.push(button_press_count);
    }

    // Map which buttons toggle which lights
    for (button_press_count, button_indices) in
        buttons_pressed.iter().zip(config.toggles_indices.iter())
    {
        for &light_index in button_indices {
            light_togglers[light_index].push(&button_press_count);
        }
    }

    // For each light, assert its final state
    for (i, &light_on) in config.lights_on.iter().enumerate() {
        let light_sum = ast::Int::new_const(format!("light_sum_{}", i));
        optimizer.assert(&light_sum.eq(&ast::Int::add(&light_togglers[i])));
        optimizer.assert(&light_sum.modulo(2).eq(if light_on { 1 } else { 0 }));
    }

    optimizer.minimize(&ast::Int::add(&buttons_pressed));

    if optimizer.check(&[]) != z3::SatResult::Sat {
        panic!("No solution found");
    }

    let model = optimizer.get_model().unwrap();
    let total_presses: i64 = buttons_pressed
        .iter()
        .map(|button| model.eval(button, true).unwrap().as_i64().unwrap())
        .sum();

    total_presses as i32
}

fn solve_subproblem2(config: Config) -> i32 {
    let optimizer = Optimize::new();
    let mut joltage_incrementers: Vec<Vec<&ast::Int>> = vec![Vec::new(); config.joltages.len()];

    let mut buttons_pressed: Vec<ast::Int> = Vec::new();
    for i in 0..config.toggles_indices.len() {
        let button_press_count = ast::Int::new_const(format!("button_{}", i));
        optimizer.assert(&button_press_count.ge(0));
        buttons_pressed.push(button_press_count);
    }

    // Map which buttons toggle which lights
    for (button_press_count, button_indices) in
        buttons_pressed.iter().zip(config.toggles_indices.iter())
    {
        for &joltage_index in button_indices {
            joltage_incrementers[joltage_index].push(&button_press_count);
        }
    }

    // For each joltage, check the sum
    for (i, &joltage) in config.joltages.iter().enumerate() {
        let joltage_sum = ast::Int::new_const(format!("joltage_sum_{}", i));
        optimizer.assert(&joltage_sum.eq(&ast::Int::add(&joltage_incrementers[i])));
        optimizer.assert(&joltage_sum.eq(joltage as i64));
    }

    optimizer.minimize(&ast::Int::add(&buttons_pressed));

    if optimizer.check(&[]) != z3::SatResult::Sat {
        panic!("No solution found");
    }

    let model = optimizer.get_model().unwrap();
    let total_presses: i64 = buttons_pressed
        .iter()
        .map(|button| model.eval(button, true).unwrap().as_i64().unwrap())
        .sum();

    total_presses as i32
}

pub fn part1(input: &str) -> i32 {
    let configs = parse_input(input);
    let mut total_presses = 0;
    for config in configs {
        total_presses += solve_subproblem(config);
    }
    total_presses
}

pub fn part2(input: &str) -> i32 {
    let configs = parse_input(input);
    let mut total_presses = 0;
    for config in configs {
        total_presses += solve_subproblem2(config);
    }
    total_presses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let config = parse_line(input);
        assert_eq!(vec![false, true, true, false], config.lights_on);
        assert_eq!(
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ],
            config.toggles_indices
        );
        assert_eq!(vec![3, 5, 4, 7], config.joltages);
    }

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("day10/example1.txt").expect("Example file not found");
        assert_eq!(7, part1(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("day10/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day10/example1.txt").expect("Example file not found");
        assert_eq!(33, part2(&input));
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day10/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
