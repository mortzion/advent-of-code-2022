use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    let max = input
        .lines()
        .fold((0, 0), |(max, current), line| match line.is_empty() {
            true => (max, 0),
            false => {
                let value: u32 = line.parse().unwrap();
                let new_current = current + value;

                (max.max(new_current), new_current)
            }
        });

    Some(max.0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut heap: BinaryHeap<u32> = BinaryHeap::from([0, 0, 0]);

    input
        .split("\n\n")
        .map(|elf_input| {
            elf_input
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .for_each(|elf| {
            if heap.peek().unwrap() < &elf {
                heap.pop();
                heap.push(elf)
            }
        });

    Some(heap.into_iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
