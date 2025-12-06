use advent_of_code_2025::Solvable;

pub struct DayTwo {
    input: String,
}

impl DayTwo {
    pub fn new() -> Self {
        Self {
            input: include_str!("../../data/two.txt").to_string(),
        }
    }
}

impl Solvable for DayTwo {
    fn solve(&self) {
        println!("[Day Two] Part 1: {}", part1(&self.input));
        println!("[Day Two] Part 2: {}", part2(&self.input));
    }

    fn solve_part1(&self) {
        println!("[Day Two] Part 1: {}", part1(&self.input));
    }

    fn solve_part2(&self) {
        println!("[Day Two] Part 2: {}", part2(&self.input));
    }
}

fn part1(input: &str) -> i64 {
    input.split(',').fold(0_i64, |sum, line| {
        let (start, end) = parse_line(line).unwrap();

        let range_sum = (start..=end)
            .filter(|&id| is_invalid_id_part_one(id))
            .map(|id| id as i64)
            .sum::<i64>();

        sum + range_sum
    })
}

fn part2(input: &str) -> i64 {
    input.split(',').fold(0_i64, |sum, line| {
        let (start, end) = parse_line(line).unwrap();

        let range_sum = (start..=end).fold(0_i64, |acc, id| {
            if is_invalid_id_part_two(id) {
                acc + id as i64
            } else {
                acc
            }
        });

        sum + range_sum
    })
}

fn is_invalid_id_part_one(id: i64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let (a, b) = s.split_at(half);
    a == b
}

fn is_invalid_id_part_two(id: i64) -> bool {
    let s = id.to_string();
    let n = s.len();

    if n < 2 {
        return false;
    }

    for p in 1..=n / 2 {
        if n % p != 0 {
            continue;
        }

        let pattern = &s[..p];
        let repeats = n / p;
        if repeats >= 2 && s == pattern.repeat(repeats) {
            return true;
        }
    }

    false
}

fn parse_line(input: &str) -> Result<(i64, i64), String> {
    let mut p = input.splitn(2, '-');

    let start = p
        .next()
        .ok_or("missing start")?
        .trim()
        .parse()
        .map_err(|_| "invalid start")?;

    let end = p
        .next()
        .ok_or("missing end")?
        .trim()
        .parse()
        .map_err(|_| "invalid end")?;

    if start > end {
        return Err(format!(
            "Start value ({start}) must be <= end value ({end}) in '{input}'"
        ));
    }

    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(part1(SAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(part2(SAMPLE_INPUT), 4174379265);
    }

    #[test]
    fn test_is_invalid_id_part_2_at_least_twice() {
        assert!(is_invalid_id_part_two(12341234)); // 1234 x2
        assert!(is_invalid_id_part_two(123123123)); // 123 x3
        assert!(is_invalid_id_part_two(1212121212)); // 12 x5
        assert!(is_invalid_id_part_two(1111111)); // 1 x7

        assert!(!is_invalid_id_part_two(12345)); // no repetition pattern
        assert!(!is_invalid_id_part_two(101)); // not a repeated block
    }
}
