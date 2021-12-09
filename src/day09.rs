use std::collections::HashSet;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|s| s.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

#[aoc(day9, part1)]
fn solve_part1(grid: &Vec<Vec<u32>>) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            let h = grid[y][x];

            if (y == 0 || grid[y - 1][x] > h)
                && (y == height - 1 || grid[y + 1][x] > h)
                && (x == 0 || grid[y][x - 1] > h)
                && (x == width - 1 || grid[y][x + 1] > h)
            {
                total += h + 1;
            }
        }
    }

    total
}

fn floodfill(x: usize, y: usize, grid: &Vec<Vec<u32>>, visited: &mut HashSet<(usize, usize)>) {
    if visited.contains(&(x, y)) {
        return;
    }

    if grid[y][x] == 9 {
        return;
    }

    visited.insert((x, y));

    let height = grid.len();
    let width = grid[0].len();
    if y != 0 {
        floodfill(x, y - 1, grid, visited);
    }
    if y != height - 1 {
        floodfill(x, y + 1, grid, visited);
    }
    if x != 0 {
        floodfill(x - 1, y, grid, visited);
    }
    if x != width - 1 {
        floodfill(x + 1, y, grid, visited);
    }
}

#[aoc(day9, part2)]
fn solve_part2(grid: &Vec<Vec<u32>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut basins = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let h = grid[y][x];

            if (y == 0 || grid[y - 1][x] > h)
                && (y == height - 1 || grid[y + 1][x] > h)
                && (x == 0 || grid[y][x - 1] > h)
                && (x == width - 1 || grid[y][x + 1] > h)
            {
                // Found a low point.
                let mut visited = HashSet::new();
                floodfill(x, y, grid, &mut visited);
                basins.push(visited.len());
            }
        }
    }

    basins.sort();
    basins.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(solve_part1(&parse(input)), 15);
        assert_eq!(solve_part2(&parse(input)), 1134);
    }
}
