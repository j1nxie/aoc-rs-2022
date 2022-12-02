pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for line in input.lines() {
        let (opponent, response) = line.split_once(" ").unwrap();
        let (left, right): (i32, i32);

        match opponent {
            "A" => left = 1,
            "B" => left = 2,
            "C" => left = 3,
            _ => unreachable!(),
        }

        match response {
            "X" => {
                right = 1;
                score += right as u32;
            }

            "Y" => {
                right = 2;
                score += right as u32;
            }

            "Z" => {
                right = 3;
                score += right as u32;
            }
            _ => unreachable!(),
        }

        if (right - left == 1) || (right - left == -2) {
            score += 6;
        }

        if right - left == 0 {
            score += 3;
        }
    }

    Some(score)
}

#[derive(PartialEq)]
enum MatchVal {
    Win(i32),
    Draw(i32),
    Lose(i32),
}

impl MatchVal {
    fn matches(&self) -> i32 {
        match self {
            MatchVal::Win(left) => {
                if left + 1 > 3 {
                    left - 2
                } else {
                    left + 1
                }
            }
            MatchVal::Draw(left) => *left,
            MatchVal::Lose(left) => {
                if left - 1 == 0 {
                    left + 2
                } else {
                    left - 1
                }
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    for line in input.lines() {
        let (opponent, response) = line.split_once(" ").unwrap();

        let left = match opponent {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!(),
        };

        let right = match response {
            "X" => MatchVal::Lose(left),

            "Y" => {
                score += 3;
                MatchVal::Draw(left)
            }

            "Z" => {
                score += 6;
                MatchVal::Win(left)
            }

            _ => unreachable!(),
        };

        match right.matches() {
            1 => score += 1,
            2 => score += 2,
            3 => score += 3,
            _ => unreachable!(),
        }
    }

    Some(score)
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
}
