fn match_outcome_by_shapes(enemy_shape: &u8, my_shape: &u8) -> u32 {
    ((my_shape - enemy_shape - 1) % 3 * 3) as u32
}

fn shape_outcome_by_shape(my_shape: &u8) -> u32 {
    *my_shape as u32 - 87
}

pub fn part_one(input: &str) -> Option<u32> {
    let points = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|splitted_line| {
            (
                splitted_line[0].bytes().next().unwrap(),
                splitted_line[1].bytes().next().unwrap(),
            )
        })
        .map(|(enemy, mine)| match_outcome_by_shapes(&enemy, &mine) + shape_outcome_by_shape(&mine))
        .sum();

    Some(points)
}

fn match_outcome_by_result(instruction: &u8) -> u32 {
    return ((instruction - 88) * 3) as u32;
}

fn shape_outcome_by_result(enemy_shape: &u8, instruction: &u8) -> u32 {
    let points: Vec<u32> = vec![2, 1, 3];

    points[((instruction - enemy_shape + instruction - 88) % 3) as usize]
}

pub fn part_two(input: &str) -> Option<u32> {
    let points = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|splitted_line| {
            (
                splitted_line[0].bytes().next().unwrap(),
                splitted_line[1].bytes().next().unwrap(),
            )
        })
        .map(|(enemy_shape, instruction)| {
            match_outcome_by_result(&instruction)
                + shape_outcome_by_result(&enemy_shape, &instruction)
        })
        .sum();

    Some(points)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_match_outcome_by_shapes() {
        fn get_first_byte(string: &str) -> u8 {
            string.as_bytes()[0]
        }

        let tests = vec![
            ("A", "X", 3),
            ("B", "X", 0),
            ("C", "X", 6),
            ("A", "Y", 6),
            ("B", "Y", 3),
            ("C", "Y", 0),
            ("A", "Z", 0),
            ("B", "Z", 6),
            ("C", "Z", 3),
        ];
    }
}
