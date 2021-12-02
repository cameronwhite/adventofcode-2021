#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_part1(entries: &Vec<i32>) -> usize {
    entries
        .windows(2)
        .filter(|pair| (pair[1] - pair[0]) > 0)
        .count()
}

#[aoc(day1, part2)]
fn solve_part2(entries: &Vec<i32>) -> usize {
    entries
        .windows(3)
        .map(|triple| triple.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|pair| (pair[1] - pair[0]) > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert_eq!(solve_part1(&parse(input)), 7);
        assert_eq!(solve_part2(&parse(input)), 5);
    }
}
