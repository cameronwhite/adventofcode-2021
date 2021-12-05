use parse_display::FromStr;
use std::collections::HashMap;

#[derive(FromStr, Debug, Clone, Hash, PartialEq, Eq)]
#[display("{x},{y}")]
struct Point {
    x: i32,
    y: i32,
}

#[derive(FromStr, Debug, Clone)]
#[display("{p1} -> {p2}")]
struct Line {
    p1: Point,
    p2: Point,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<Line> {
    input.lines().map(|s| s.parse::<Line>().unwrap()).collect()
}

#[aoc(day5, part1)]
fn solve_part1(lines: &Vec<Line>) -> usize {
    let mut grid = HashMap::new();

    for line in lines {
        let x_step = (line.p2.x - line.p1.x).signum();
        let y_step = (line.p2.y - line.p1.y).signum();

        if x_step != 0 && y_step != 0 {
            continue;
        }

        let mut x = line.p1.x;
        let mut y = line.p1.y;
        loop {
            *grid.entry(Point { x, y }).or_insert(0) += 1;
            if x == line.p2.x && y == line.p2.y {
                break;
            }

            x += x_step;
            y += y_step;
        }
    }

    grid.values().filter(|&c| *c >= 2).count()
}

#[aoc(day5, part2)]
fn solve_part2(lines: &Vec<Line>) -> usize {
    let mut grid = HashMap::new();

    for line in lines {
        let x_step = (line.p2.x - line.p1.x).signum();
        let y_step = (line.p2.y - line.p1.y).signum();

        let mut x = line.p1.x;
        let mut y = line.p1.y;
        loop {
            *grid.entry(Point { x, y }).or_insert(0) += 1;
            if x == line.p2.x && y == line.p2.y {
                break;
            }

            x += x_step;
            y += y_step;
        }
    }

    grid.values().filter(|&c| *c >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(solve_part1(&parse(input)), 5);
        assert_eq!(solve_part2(&parse(input)), 12);
    }
}
