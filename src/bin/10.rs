use std::collections::HashMap;

fn noop(cycle: &mut i32) {
    *cycle += 1;
}

fn addx(cycle: &mut i32, x: &mut i32, value: i32) {
    *cycle += 1;
    *x += value;
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut ans = 0;
    let mut cycle = 1;
    let mut x = 1;
    for line in input.lines() {
        let command: Vec<&str> = line.split(' ').collect();
        match command.len() {
            1 => {
                noop(&mut cycle);
            }

            2 => {
                let value = command[1].parse::<i32>().unwrap();
                noop(&mut cycle);
                if cycle % 40 == 20 && cycle <= 220 {
                    ans += cycle * x;
                }
                addx(&mut cycle, &mut x, value);
            }

            _ => unreachable!(),
        }

        if cycle % 40 == 20 && cycle <= 220 {
            ans += cycle * x;
        }
    }
    Some(ans)
}

fn write_vram(cycle: &mut i32, x: i32, pixels: &mut HashMap<i32, char>) {
    let pos = *cycle % 40;
    if ((x - 1)..=(x + 1)).contains(&pos) {
        pixels.insert(*cycle, '#');
    } else {
        pixels.insert(*cycle, '.');
    }
    *cycle += 1;
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut cycle = 0;
    let mut x = 1;
    let mut pixels: HashMap<i32, char> = HashMap::new();

    for line in input.lines() {
        let command: Vec<&str> = line.split(' ').collect();
        write_vram(&mut cycle, x, &mut pixels);

        if command[0] == "addx" {
            let value = command[1].parse::<i32>().unwrap();
            write_vram(&mut cycle, x, &mut pixels);
            x += value;
        }
    }

    for i in 0..240 {
        eprint!("{}", pixels[&i]);
        if i % 40 == 0 {
            eprintln!();
        }
    }

    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(0));
    }
}
