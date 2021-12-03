#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect()
}

fn compute_num_bits(numbers: &Vec<u32>) -> u32 {
    numbers
        .iter()
        .map(|n| u32::BITS - u32::leading_zeros(*n))
        .max()
        .unwrap()
}

fn most_common_bit(numbers: &Vec<u32>, test_bit: u32) -> bool {
    let num_ones = numbers.iter().filter(|n| (*n & test_bit) != 0).count();
    num_ones >= numbers.len() - num_ones
}

fn least_common_bit(numbers: &Vec<u32>, test_bit: u32) -> bool {
    !most_common_bit(numbers, test_bit)
}

#[aoc(day3, part1)]
fn solve_part1(numbers: &Vec<u32>) -> u32 {
    let num_bits = compute_num_bits(numbers);

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in 0..num_bits {
        let test_bit = 1 << i;
        match most_common_bit(numbers, test_bit) {
            true => gamma |= test_bit,
            false => epsilon |= test_bit,
        }
    }

    gamma * epsilon
}

fn compute_rating(mut numbers: Vec<u32>, i: u32, bit_criteria: fn(&Vec<u32>, u32) -> bool) -> u32 {
    let test_bit = 1 << i;
    let bit_to_keep = bit_criteria(&numbers, test_bit);

    numbers.retain(|n| ((n & test_bit) != 0) == bit_to_keep);

    if numbers.len() <= 1 || i == 0 {
        numbers[0]
    } else {
        compute_rating(numbers, i - 1, bit_criteria)
    }
}

#[aoc(day3, part2)]
fn solve_part2(numbers: &Vec<u32>) -> u32 {
    let num_bits = compute_num_bits(numbers);

    compute_rating(numbers.clone(), num_bits - 1, most_common_bit)
        * compute_rating(numbers.clone(), num_bits - 1, least_common_bit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        assert_eq!(solve_part1(&parse(input)), 198);
        assert_eq!(solve_part2(&parse(input)), 230);
    }
}
