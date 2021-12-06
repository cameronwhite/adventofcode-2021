#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn simulate(times: &Vec<usize>, days: i32) -> usize {
    let mut state = [0; 9];

    for &time in times {
        state[time] += 1;
    }

    for _ in 0..days {
        state.rotate_left(1);
        state[6] += state[8];
    }

    state.iter().sum()
}

#[aoc(day6, part1)]
fn solve_part1(times: &Vec<usize>) -> usize {
    simulate(times, 80)
}

#[aoc(day6, part2)]
fn solve_part2(times: &Vec<usize>) -> usize {
    simulate(times, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "3,4,3,1,2";

        assert_eq!(solve_part1(&parse(input)), 5934);
        assert_eq!(solve_part2(&parse(input)), 26984457539);
    }
}
