const START_NUMBER: i32 = 50;

pub fn part1() -> i32 {
    let input = include_str!("../data/one.txt");
    solve_part1(input)
}

pub fn part2() -> i32 {
    let input = include_str!("../data/one.txt");
    solve_part2(input)
}

pub fn solve_part1(input: &str) -> i32 {
    let (sequence, password) = input
        .lines()
        .fold((Vec::new(), 0), |(mut acc, mut count), line| {
            let (dir, dist) = parse_instruction(line).unwrap();
            let step = dir * dist;

            let previous_number = if acc.is_empty() {
                START_NUMBER
            } else {
                *acc.last().unwrap()
            };

            let next = (previous_number + step).rem_euclid(100);

            if next == 0 {
                count += 1;
            }

            acc.push(next);

            (acc, count)
        });

    println!(
        "String: {}",
        sequence
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    println!("Password: {}", password);

    password
}

pub fn solve_part2(input: &str) -> i32 {
    let (sequence, password) = input
        .lines()
        .fold((Vec::new(), 0), |(mut acc, mut count), line| {
            let (dir, dist) = parse_instruction(line).unwrap();
            let step = dir * dist;

            let previous_number = if acc.is_empty() {
                START_NUMBER
            } else {
                *acc.last().unwrap()
            };

            let step = dir * dist;

            let hits = zero_crossings(previous_number, step);
            count += hits;

            let next = (previous_number + step).rem_euclid(100);
            acc.push(next);

            (acc, count)
        });

    println!(
        "String: {}",
        sequence
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    println!("Password: {}", password);

    password
}

fn parse_instruction(line: &str) -> Result<(i32, i32), String> {
    if line.len() < 2 {
        return Err(format!("Line too short: {line}"));
    }

    let (turn, steps) = line.split_at(1);

    let dir = match turn.chars().next().unwrap() {
        'L' => -1,
        'R' => 1,
        other => return Err(format!("Invalid direction '{other}' in line: {line}")),
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
    let mut count = 0;
    let mut pos = prev;

    let dir = step.signum();
    let steps = step.abs();

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

        let result = solve_part1(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_moderate_large_distances() {
        let input = "L120\nR85\nL300\nR40\nL260";

        // Manual expected behaviour:
        //
        // start = 50
        //
        // L120 → (50 - 120) % 100 = 30
        // R85  → (30 + 85) % 100 = 15
        // L300 → (15 - 300) % 100 = 15
        // R40  → (15 + 40) % 100 = 55
        // L260 → (55 - 260) % 100 = 95
        //
        // Sequence: [30, 15, 15, 55, 95]
        // Zero hits: 0

        let result = solve_part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_sample_input() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

        let result = solve_part2(input);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2_large_number() {
        let input = "R1000";

        let result = solve_part2(input);

        assert_eq!(result, 10);
    }
}
