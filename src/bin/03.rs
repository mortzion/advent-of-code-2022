use std::{
    collections::{hash_map::RandomState, HashSet},
    hash::Hash,
};

fn to_compartments(line: &[u8]) -> (&[u8], &[u8]) {
    line.split_at(line.len() / 2)
}

fn to_set(bytes: &[u8]) -> HashSet<&u8> {
    HashSet::from_iter(bytes.iter())
}

fn to_priority(byte: &u8) -> u32 {
    if byte > &96 {
        (byte - 96) as u32
    } else {
        (byte - 38) as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let compartments = to_compartments(line.as_bytes());

            to_set(compartments.0)
                .intersection(&to_set(compartments.1))
                .map(|shared_type| to_priority(shared_type))
                .sum::<u32>()
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|line| -> HashSet<u8, RandomState> { HashSet::from_iter(line.bytes()) })
        })
        .map(|group_sets| group_sets.reduce(|bag1, bag2| &bag1 & &bag2).unwrap())
        .map(|shared_types| {
            shared_types
                .iter()
                .map(|shared_type| to_priority(shared_type))
                .sum::<u32>()
        })
        .sum::<u32>();

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn test_to_priority() {
        assert_eq!(to_priority(&b'a'), 1);
        assert_eq!(to_priority(&b'z'), 26);
        assert_eq!(to_priority(&b'A'), 27);
        assert_eq!(to_priority(&b'Z'), 52);
    }
}
