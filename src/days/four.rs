use advent_of_code_2025::Day;

pub struct DayFour {
    input: String,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<i8>,
}

impl DayFour {
    pub fn new() -> Self {
        Self {
            input: include_str!("../../data/four.txt").to_string(),
        }
    }
}

impl Day<i32> for DayFour {
    const NAME: &'static str = "Day Four";

    fn part1(&self) -> i32 {
        part1(&self.input)
    }

    fn part2(&self) -> i32 {
        part2(&self.input)
    }
}

fn part1(input: &str) -> i32 {
    let grid = parse_grid(input);

    grid.data.iter().enumerate().fold(0, |mut acc, (i, space)| {
        if *space > 0 && count_roll_neighbours(i, &grid) < 4 {
            acc += 1;
        }

        acc
    })
}

use std::collections::HashSet;

// Flow:
// 1. Create set of rolls to check. On first pass this is all.
// 2. For all rolls that pass the check, add their neighbours to next check set
// 3. Remove all rolls
// 4. Repeat
fn part2(input: &str) -> i32 {
    let mut grid = parse_grid(input);
    let mut total_removed = 0;

    let mut to_check: HashSet<usize> = (0..grid.data.len())
        .filter(|&i| grid.data[i] == 1)
        .collect();

    while !to_check.is_empty() {
        let mut to_remove = Vec::new();
        let mut next_candidates = HashSet::new();

        for &i in &to_check {
            let neighbours = get_roll_neighbours(i, &grid);
            if grid.data[i] == 1 && neighbours.len() < 4 {
                to_remove.push(i);

                for ni in neighbours {
                    if grid.data[ni] == 1 {
                        next_candidates.insert(ni);
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for &i in &to_remove {
            grid.data[i] = 0;
        }

        total_removed += to_remove.len();
        to_check = next_candidates;
    }

    total_removed as i32
}

fn get_roll_neighbours(i: usize, grid: &Grid) -> Vec<usize> {
    let w = grid.width;
    let h = grid.height;
    let x = i % w;
    let y = i / w;

    let mut neighbours = Vec::with_capacity(8);

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // skip the cell itself
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < w as isize && ny >= 0 && ny < h as isize {
                let ni = idx(nx as usize, ny as usize, w);
                if grid.data[ni] == 1 {
                    neighbours.push(ni);
                }
            }
        }
    }

    neighbours
}

fn count_roll_neighbours(i: usize, grid: &Grid) -> i32 {
    let neighbours = get_roll_neighbours(i, grid);

    neighbours.len() as i32
}

fn idx(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn parse_grid(input: &str) -> Grid {
    let lines: Vec<&str> = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut data = Vec::with_capacity(width * height);

    for line in &lines {
        assert_eq!(line.len(), width, "Grid must be rectangular");
        for c in line.chars() {
            data.push(if c == '@' { 1 } else { 0 });
        }
    }

    Grid {
        width,
        height,
        data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_parse_grid() {
        let grid = parse_grid(SAMPLE);
        assert_eq!(
            grid.data,
            [
                0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0,
                1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1,
                1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1,
                1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0
            ]
        );

        assert_eq!(grid.height, 10);
        assert_eq!(grid.width, 10);
    }

    #[test]
    fn test_part1_sample_input() {
        assert_eq!(part1(SAMPLE), 13)
    }

    #[test]
    fn test_part2_sample_input() {
        assert_eq!(part2(SAMPLE), 43)
    }
}
