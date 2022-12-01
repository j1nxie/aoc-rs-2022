pub fn part_one(input: &str) -> Option<u32> {
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut foods: Vec<u32> = vec![];
    for elf in elves {
        let food: u32 = elf.lines().map(|x| x.parse::<u32>().unwrap()).sum();
        foods.push(food);
    }

    Some(foods.into_iter().max().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves: Vec<&str> = input.split("\n\n").collect();
    let mut foods: Vec<u32> = vec![];
    for elf in elves {
        let food: u32 = elf.lines().map(|x| x.parse::<u32>().unwrap()).sum();
        foods.push(food);
    }

    foods.sort_by(|a, b| b.cmp(a));
    Some(foods[0] + foods[1] + foods[2])
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
