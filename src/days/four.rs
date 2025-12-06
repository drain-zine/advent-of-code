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
        if *space > 0 && count_neighbors(i, &grid) < 4 {
            acc += 1;
        }

        acc
    })
}

fn part2(input: &str) -> i32 {
    let mut grid = parse_grid(input);
    let mut total_removed = 0;

    loop {
        let mut removed = 0;

        for i in 0..grid.data.len() {
            if grid.data[i] == 1 && count_neighbors(i, &grid) < 4 {
                grid.data[i] = 0;
                removed += 1;
            }
        }

        if removed == 0 {
            break;
        }

        total_removed += removed;
    }

    total_removed
}

fn count_neighbors(i: usize, grid: &Grid) -> i32 {
    let width = grid.width;
    let height = grid.height;

    let x = i % width;
    let y = i / width;

    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // skip centre
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let ni = idx(nx as usize, ny as usize, width);
                count += grid.data[ni] as i32;
            }
        }
    }

    count
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
