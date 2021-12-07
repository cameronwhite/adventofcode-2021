#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn solve_part1(positions: &Vec<i32>) -> i32 {
    let mut ordered = positions.clone();
    ordered.sort();

    let median = ordered[ordered.len() / 2];
    positions.iter().map(|p| (p - median).abs()).sum()
}

fn cost(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .map(|p| {
            let diff = (p - target).abs();
            diff * (diff + 1) / 2
        })
        .sum()
}

#[aoc(day7, part2)]
fn solve_part2(positions: &Vec<i32>) -> i32 {
    let sum = positions.iter().sum::<i32>() as f64;
    let mean = sum / (positions.len() as f64);
    let mean_floor = mean.floor() as i32;
    let mean_ceil = mean.ceil() as i32;

    std::cmp::min(cost(positions, mean_floor), cost(positions, mean_ceil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(solve_part1(&parse(input)), 37);
        assert_eq!(solve_part2(&parse(input)), 168);
    }
}
