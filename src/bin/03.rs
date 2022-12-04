use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut answer: u32 = 0;
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let mut a = HashSet::new();
        let mut b = HashSet::new();

        for c in first.chars() {
            a.insert(c);
        }

        for c in second.chars() {
            b.insert(c);
        }

        let intersect = a.intersection(&b).next().unwrap();

        match intersect {
            'a'..='z' => answer += *intersect as u32 - 96,
            _ => answer += *intersect as u32 - 64 + 26,
        }
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .step_by(3)
        .zip(
            input
                .lines()
                .skip(1)
                .step_by(3)
                .zip(input.lines().skip(2).step_by(3)),
        )
        .flat_map(|(first, (second, third))| {
            first
                .chars()
                .find(|c| second.contains(*c) && third.contains(*c))
        })
        .collect::<String>()
        .chars()
        .fold(0, |acc, c| {
            if c.is_ascii_lowercase() {
                acc + (c as u32) - 96
            } else {
                acc + (c as u32) - 38
            }
        });
    Some(answer)
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
}
