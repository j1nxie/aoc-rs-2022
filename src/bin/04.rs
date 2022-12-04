pub fn part_one(input: &str) -> Option<u32> {
    let mut answer = 0;
    for line in input.lines() {
        let (first, second) = line.split_once(',').unwrap();
        let (first_start, first_end) = first.split_once('-').unwrap();
        let (second_start, second_end) = second.split_once('-').unwrap();

        let first_start_n = first_start.parse::<u32>().unwrap();
        let first_end_n = first_end.parse::<u32>().unwrap();
        let second_start_n = second_start.parse::<u32>().unwrap();
        let second_end_n = second_end.parse::<u32>().unwrap();

        if (first_start_n <= second_start_n) && (first_end_n >= second_end_n)
            || (first_start_n >= second_start_n) && (first_end_n <= second_end_n)
        {
            answer += 1;
        }
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut answer = 0;
    for line in input.lines() {
        let (first, second) = line.split_once(',').unwrap();
        let (first_start, first_end) = first.split_once('-').unwrap();
        let (second_start, second_end) = second.split_once('-').unwrap();

        let first_start_n = first_start.parse::<u32>().unwrap();
        let first_end_n = first_end.parse::<u32>().unwrap();
        let second_start_n = second_start.parse::<u32>().unwrap();
        let second_end_n = second_end.parse::<u32>().unwrap();

        if (first_start_n <= second_start_n) && (first_end_n >= second_end_n)
            || (first_start_n >= second_start_n) && (first_end_n <= second_end_n)
            || (first_end_n >= second_start_n) && (first_end_n <= second_end_n)
            || (first_end_n <= second_start_n) && (first_end_n >= second_end_n)
            || (first_start_n >= second_end_n) && (first_start_n <= second_start_n)
            || (first_start_n <= second_end_n) && (first_start_n >= second_start_n)
        {
            answer += 1;
        }
    }
    Some(answer)
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
}
