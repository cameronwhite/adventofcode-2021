use std::cmp::{max, min};
use std::collections::VecDeque;

type Grid = Vec<Vec<u32>>;

#[aoc_generator(day11)]
fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn step(grid: &mut Grid) -> usize {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut queue = VecDeque::new();
    let mut count = 0;

    for y in 0..h {
        for x in 0..w {
            let cell = &mut grid[y as usize][x as usize];
            *cell += 1;

            if *cell > 9 {
                *cell = 0;
                count += 1;
                queue.push_back((x, y));
            }
        }
    }

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        let x_start = max(x - 1, 0);
        let x_end = min(x + 2, w);
        let y_start = max(y - 1, 0);
        let y_end = min(y + 2, h);

        for y in y_start..y_end {
            for x in x_start..x_end {
                let neighbor = &mut grid[y as usize][x as usize];
                if *neighbor != 0 {
                    *neighbor += 1;
                    if *neighbor > 9 {
                        *neighbor = 0;
                        count += 1;
                        queue.push_back((x, y));
                    }
                }
            }
        }
    }

    count
}

#[aoc(day11, part1)]
fn solve_part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();

    let mut count = 0;
    for _ in 0..100 {
        count += step(&mut grid);
    }

    count
}

#[aoc(day11, part2)]
fn solve_part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();

    let h = grid.len();
    let w = grid[0].len();
    let num_cells = h * w;

    let mut i = 0;
    loop {
        i += 1;
        if step(&mut grid) == num_cells {
            break;
        }
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(solve_part1(&parse(input)), 1656);
        assert_eq!(solve_part2(&parse(input)), 195);
    }
}
