use advent_of_code_2025::Solvable;

const START_NUMBER: i32 = 50;

pub struct DayOne {
    input: String,
}

impl DayOne {
    pub fn new() -> Self {
        Self {
            input: include_str!("../data/one.txt").to_string(),
        }
    }
}

impl Solvable for DayOne {
    fn solve(&self) {
        println!("[Day One] Part 1: {}", part1(&self.input));
        println!("[Day One] Part 2: {}", part2(&self.input));
    }

    fn solve_part1(&self) {
        println!("[Day One] Part 1: {}", part1(&self.input));
    }

    fn solve_part2(&self) {
        println!("[Day One] Part 2: {}", part2(&self.input));
    }
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .fold((START_NUMBER, 0), |(prev, count), line| {
            let (dir, dist) = parse_instruction(line).unwrap();
            let next = (prev + dir * dist).rem_euclid(100);

            let count = count + if next == 0 { 1 } else { 0 };
            (next, count)
        })
        .1
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .fold((START_NUMBER, 0), |(prev, count), line| {
            let (dir, dist) = parse_instruction(line).unwrap();
            let step = dir * dist;

            let hits = zero_crossings(prev, step);
            let next = (prev + step).rem_euclid(100);

            (next, count + hits)
        })
        .1
}

fn parse_instruction(line: &str) -> Result<(i32, i32), String> {
    if line.len() < 2 {
        return Err(format!("Line too short: {line}"));
    }

    let (turn, steps) = line.split_at(1);

    let dir = match turn.as_bytes()[0] {
        b'L' => -1,
        b'R' => 1,
        other => {
            return Err(format!(
                "Invalid direction '{}' in line: {line}",
                other as char
            ));
        }
    };

    let dist: i32 = steps
        .parse()
        .map_err(|_| format!("Invalid number '{}' in line: {line}", steps))?;

    if dist <= 0 {
        return Err(format!("Steps must be > 0 (got {}) in line: {line}", dist));
    }

    Ok((dir, dist))
}

fn zero_crossings(prev: i32, step: i32) -> i32 {
    let mut pos = prev;
    let dir = step.signum();
    let steps = step.abs();

    let mut count = 0;

    for _ in 0..steps {
        pos = (pos + dir).rem_euclid(100);
        if pos == 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample_input() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part1_moderate_large_distances() {
        let input = "L120\nR85\nL300\nR40\nL260";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2_sample_input() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

        assert_eq!(part2(input), 6);
    }

    #[test]
    fn test_part2_large_number() {
        let input = "R1000";
        assert_eq!(part2(input), 10);
    }
}
