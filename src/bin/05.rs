use std::{
    iter::{repeat, Peekable},
    str::Lines,
};

use regex::Regex;

pub fn parse_stacks(lines: &mut Peekable<Lines>) -> Vec<Vec<char>> {
    let count_stacks = (lines.peek().unwrap().len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = repeat(vec![]).take(count_stacks).collect();
    let stack_lines = lines
        .take_while(|line| !line.starts_with(" 1 "))
        .collect::<Vec<&str>>();

    for line in stack_lines.iter().rev() {
        let chars = line.chars().collect::<Vec<char>>();

        chars
            .chunks(4)
            .zip(stacks.iter_mut())
            .for_each(|(stack_chars, stack)| {
                if !stack_chars[1].is_whitespace() {
                    stack.push(stack_chars[1]);
                }
            });
    }

    lines.next();

    stacks
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines().peekable();
    let mut stacks = parse_stacks(&mut lines);
    let regex = Regex::new(r"move (?P<count>\d*) from (?P<from>\d*) to (?P<to>\d*)").unwrap();

    for line in lines {
        let captured = regex.captures(line).unwrap();
        let count: usize = captured.name("count").unwrap().as_str().parse().unwrap();
        let from_index: usize = captured.name("from").unwrap().as_str().parse().unwrap();
        let to_index: usize = captured.name("to").unwrap().as_str().parse().unwrap();

        let from = &mut stacks[from_index - 1];
        let mut to_move = from.split_off(from.len() - count);

        to_move.reverse();

        stacks[to_index - 1].append(&mut to_move);
    }

    Some(stacks.iter().map(|stack| stack.last().unwrap()).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines = input.lines().peekable();
    let mut stacks = parse_stacks(&mut lines);
    let regex = Regex::new(r"move (?P<count>\d*) from (?P<from>\d*) to (?P<to>\d*)").unwrap();

    for line in lines {
        let captured = regex.captures(line).unwrap();
        let count: usize = captured.name("count").unwrap().as_str().parse().unwrap();
        let from_index: usize = captured.name("from").unwrap().as_str().parse().unwrap();
        let to_index: usize = captured.name("to").unwrap().as_str().parse().unwrap();

        let from = &mut stacks[from_index - 1];
        let mut to_move = from.split_off(from.len() - count);

        stacks[to_index - 1].append(&mut to_move);
    }

    Some(stacks.iter().map(|stack| stack.last().unwrap()).collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
