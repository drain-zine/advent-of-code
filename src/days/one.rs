use std::cmp::max;

use advent_of_code_2025::Day;

const START_NUMBER: i32 = 50;
const DIAL_SIZE: i32 = 100;

pub struct DayOne {
    input: String,
}

impl DayOne {
    pub fn new() -> Self {
        Self {
            input: include_str!("../../data/one.txt").to_string(),
        }
    }
}

impl Day<i32> for DayOne {
    const NAME: &'static str = "Day One";

    fn part1(&self) -> i32 {
        part1(&self.input)
    }

    fn part2(&self) -> i32 {
        part2(&self.input)
    }
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .fold((START_NUMBER, 0), |(prev, count), line| {
            let (dir, dist) = parse_instruction(line).unwrap();
            let next = (prev + dir * dist).rem_euclid(DIAL_SIZE);

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
            let next = (prev + step).rem_euclid(DIAL_SIZE);

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
    if step == 0 {
        return 0;
    }

    let step_dir = step.signum();

    let first_position_visited = prev + step_dir;
    let last_position_visited = prev + step;

    let (lo, hi) = if first_position_visited <= last_position_visited {
        (first_position_visited, last_position_visited)
    } else {
        (last_position_visited, first_position_visited)
    };

    let first_crossing_point = div_ceil(lo, DIAL_SIZE);
    let last_crossing_point = hi.div_euclid(DIAL_SIZE);

    let crossings = last_crossing_point - first_crossing_point + 1;

    max(crossings, 0)
}

fn div_ceil(n: i32, d: i32) -> i32 {
    let quotient = n.div_euclid(d);
    let remainder = n.rem_euclid(d);

    if remainder == 0 {
        quotient
    } else {
        quotient + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(part1(SAMPLE), 3);
    }

    #[test]
    fn test_part1_moderate_large_distances() {
        let input = "L120\nR85\nL300\nR40\nL260";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(part2(SAMPLE), 6);
    }

    #[test]
    fn test_part2_large_number() {
        let input = "R1000";
        assert_eq!(part2(input), 10);
    }

    #[test]
    fn test_zero_crossings() {
        assert_eq!(zero_crossings(50, 10), 0);
        assert_eq!(zero_crossings(95, 10), 1);
        assert_eq!(zero_crossings(50, 260), 3);
        assert_eq!(zero_crossings(50, -260), 3);
        assert_eq!(zero_crossings(50, 100), 1);
        assert_eq!(zero_crossings(50, -100), 1);
        assert_eq!(zero_crossings(50, 0), 0);
        assert_eq!(zero_crossings(90, 10), 1);
        assert_eq!(zero_crossings(10, -10), 1);
    }
}
