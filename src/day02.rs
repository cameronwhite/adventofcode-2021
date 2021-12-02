use parse_display::FromStr;

#[derive(FromStr, Debug, PartialEq, Clone)]
#[display(style = "lowercase")]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(FromStr, Debug, Clone)]
#[display("{dir} {dist}")]
struct Command {
    dir: Direction,
    dist: i32,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|x| x.parse::<Command>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(commands: &Vec<Command>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for cmd in commands {
        match cmd.dir {
            Direction::Forward => x += cmd.dist,
            Direction::Down => y += cmd.dist,
            Direction::Up => y -= cmd.dist,
        }
    }

    x * y
}

#[aoc(day2, part2)]
fn solve_part2(commands: &Vec<Command>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for cmd in commands {
        match cmd.dir {
            Direction::Forward => {
                x += cmd.dist;
                y += aim * cmd.dist;
            }
            Direction::Down => aim += cmd.dist,
            Direction::Up => aim -= cmd.dist,
        }
    }

    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert_eq!(solve_part1(&parse(input)), 150);
        assert_eq!(solve_part2(&parse(input)), 900);
    }
}
