#[macro_use]
extern crate lazy_static;

use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
struct Range {
    from: u32,
    to: u32,
}

impl Range {
    fn new(from: u32, to: u32) -> Self {
        Self { from, to }
    }

    fn contains(&self, other: &Range) -> bool {
        self.from >= other.from && self.to <= other.to
            || other.from >= self.from && other.to <= self.to
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.from >= other.from && self.from <= other.to
            || self.to >= other.from && self.to <= other.to
            || self.contains(other)
    }
}

fn parse_line(line: &str) -> (Range, Range) {
    lazy_static! {
        static ref LINE_PARSER: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }

    let captured = LINE_PARSER.captures(line).unwrap();

    (
        Range {
            from: captured.get(1).unwrap().as_str().parse().unwrap(),
            to: captured.get(2).unwrap().as_str().parse().unwrap(),
        },
        Range {
            from: captured.get(3).unwrap().as_str().parse().unwrap(),
            to: captured.get(4).unwrap().as_str().parse().unwrap(),
        },
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = input
        .lines()
        .map(|line| parse_line(&line))
        .filter(|(pair1, pair2)| pair1.contains(&pair2))
        .count();

    Some(pairs as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = input
        .lines()
        .map(|line| parse_line(&line))
        .filter(|(pair1, pair2)| pair1.overlaps(&pair2))
        .count();

    Some(pairs as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }

    #[test]
    fn test_parse_line() {
        let tests = [("12-13,14-15", Range::new(12, 13), Range::new(14, 15))];

        for test in tests {
            let (range1, range2) = parse_line(test.0);

            assert_eq!(range1, test.1);
            assert_eq!(range2, test.2);
        }
    }

    #[test]
    fn test_contains() {
        let tests = [
            (Range::new(0, 4), Range::new(2, 3), true),
            (Range::new(2, 3), Range::new(0, 4), true),
            (Range::new(1, 3), Range::new(2, 4), false),
        ];

        for test in tests {
            assert_eq!(test.0.contains(&test.1), test.2);
        }
    }

    #[test]
    fn test_overlaps() {
        let tests = [
            (Range::new(0, 4), Range::new(5, 6), false),
            (Range::new(5, 6), Range::new(0, 4), false),
            (Range::new(1, 3), Range::new(2, 4), true),
            (Range::new(2, 4), Range::new(1, 3), true),
        ];

        for test in tests {
            assert_eq!(
                test.0.overlaps(&test.1),
                test.2,
                "Ranges {:?} and {:?} overlaps",
                test.0,
                test.1
            );
        }
    }
}
