use regex::Regex;

fn move_item(stacks: &mut [Vec<String>], len: usize, from: usize, to: usize) {
    for _ in 0..len {
        if let Some(item) = stacks[from].pop() {
            stacks[to].push(item);
        }
    }
}

fn move_items_bulk(stacks: &mut [Vec<String>], len: usize, from: usize, to: usize) {
    let moved_stack = &mut stacks[from];
    let tail = moved_stack.split_off(moved_stack.len() - len);
    stacks[to].extend(tail);
}

pub fn part_one(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<String>> = vec![];
    let input_split: Vec<&str> = input.split("\n\n").collect();
    let input_stacks = input_split[0];
    let input_moves = input_split[1];

    for line in input_stacks.lines() {
        for (i, stack_item) in line.chars().enumerate() {
            if ('A'..='Z').contains(&stack_item) {
                let stack_index = (i + 1) / 4;
                if stacks.len() < stack_index + 1 {
                    for _ in stacks.len()..stack_index + 1 {
                        stacks.push(vec![]);
                    }
                }
                stacks[stack_index].insert(0, stack_item.to_string());
            }
        }
    }

    for line in input_moves.lines() {
        let re = Regex::new(r"(\d*)").unwrap();
        let args: Vec<usize> = re
            .captures_iter(line)
            .filter_map(|cap| cap[1].parse::<usize>().ok())
            .collect();
        move_item(&mut stacks, args[0], args[1] - 1, args[2] - 1);
    }
    Some(
        stacks
            .iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<String>> = vec![];
    let input_split: Vec<&str> = input.split("\n\n").collect();
    let input_stacks = input_split[0];
    let input_moves = input_split[1];

    for line in input_stacks.lines() {
        for (i, stack_item) in line.chars().enumerate() {
            if ('A'..='Z').contains(&stack_item) {
                let stack_index = (i + 1) / 4;
                if stacks.len() < stack_index + 1 {
                    for _ in stacks.len()..stack_index + 1 {
                        stacks.push(vec![]);
                    }
                }
                stacks[stack_index].insert(0, stack_item.to_string());
            }
        }
    }

    for line in input_moves.lines() {
        let re = Regex::new(r"(\d*)").unwrap();
        let args: Vec<usize> = re
            .captures_iter(line)
            .filter_map(|cap| cap[1].parse::<usize>().ok())
            .collect();
        move_items_bulk(&mut stacks, args[0], args[1] - 1, args[2] - 1);
    }
    Some(
        stacks
            .iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect::<String>(),
    )
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
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
