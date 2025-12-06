use advent_of_code_2025::Day;

pub struct DayThree {
    input: String,
}

impl DayThree {
    pub fn new() -> Self {
        Self {
            input: include_str!("../../data/three.txt").to_string(),
        }
    }
}

impl Day<i64> for DayThree {
    const NAME: &'static str = "Day Three";

    fn part1(&self) -> i64 {
        part1(&self.input)
    }

    fn part2(&self) -> i64 {
        part2(&self.input)
    }
}

fn part1(input: &str) -> i64 {
    input.lines().fold(0, |acc, line| acc + max_joltage(line))
}

fn max_joltage(bank: &str) -> i64 {
    let digits = parse_digits(bank);

    let mut max_digit = 0;
    let mut max_pair = 0;

    for d in digits {
        max_pair = max_pair.max(10 * max_digit + d);
        max_digit = max_digit.max(d);
    }

    max_pair
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .fold(0, |acc, line| acc + max_joltage_12(line))
}

fn max_joltage_12(bank: &str) -> i64 {
    let digits: Vec<i64> = parse_digits(bank);

    let total_digits = digits.len();
    let target_len = 12;
    let max_removals = total_digits - target_len;

    let mut chosen: Vec<i64> = Vec::with_capacity(total_digits);
    let mut removed = 0;

    for &digit in &digits {
        while let Some(&last_digit) = chosen.last() {
            if removed < max_removals && digit > last_digit {
                chosen.pop();
                removed += 1;
            } else {
                break;
            }
        }
        chosen.push(digit);
    }

    chosen.truncate(target_len);

    chosen
        .iter()
        .fold(0_i64, |value, &digit| value * 10 + digit)
}

fn parse_digits(bank: &str) -> Vec<i64> {
    bank.chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i64))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [&str; 4] = [
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(part1(SAMPLE[0]), 98);
        assert_eq!(part1(SAMPLE[1]), 89);
        assert_eq!(part1(SAMPLE[2]), 78);
        assert_eq!(part1(SAMPLE[3]), 92);
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(part2(SAMPLE[0]), 987654321111);
        assert_eq!(part2(SAMPLE[1]), 811111111119);
        assert_eq!(part2(SAMPLE[2]), 434234234278);
        assert_eq!(part2(SAMPLE[3]), 888911112111);
    }
}
