use std::collections::HashSet;

fn solve(input: &str, marker_size: usize) -> usize {
    let mut ans = 0;
    let data_buffer = input.chars().collect::<Vec<char>>();
    for (index, window) in data_buffer.windows(marker_size).enumerate() {
        if window.iter().collect::<HashSet<_>>().len() == marker_size {
            ans = index + marker_size;
            break;
        }
    }

    ans
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 4) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 14) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
