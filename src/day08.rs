use std::collections::HashSet;

type Entry = (Vec<String>, Vec<String>);

fn split_strings(src: &str) -> Vec<String> {
    src.split_whitespace().map(|s| s.to_string()).collect()
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|s| {
            let (signals, outputs) = s.split_once("|").unwrap();
            (split_strings(signals), split_strings(outputs))
        })
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(entries: &Vec<Entry>) -> usize {
    let easy_digits = HashSet::from([2, 3, 4, 7]);
    entries
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .filter(|output| easy_digits.contains(&(output.len() as i32)))
                .count()
        })
        .sum()
}

fn common_chars(s1: &str, s2: &str) -> usize {
    s1.chars().filter(|c| s2.contains(*c)).count()
}

#[aoc(day8, part2)]
fn solve_part2(entries: &Vec<Entry>) -> i32 {
    entries
        .iter()
        .map(|(patterns, outputs)| {
            // Can uniquely determine based on string length and common characters with the 1 & 4
            // patterns.
            let pattern_1 = patterns.iter().find(|s| s.len() == 2).unwrap();
            let pattern_4 = patterns.iter().find(|s| s.len() == 4).unwrap();

            outputs
                .iter()
                .rev()
                .enumerate()
                .map(|(i, digit)| {
                    10_i32.pow(i as u32)
                        * match (
                            digit.len(),
                            common_chars(digit, pattern_1),
                            common_chars(digit, pattern_4),
                        ) {
                            (6, 2, 3) => 0,
                            (2, 2, 2) => 1,
                            (5, 1, 2) => 2,
                            (5, 2, 3) => 3,
                            (4, 2, 4) => 4,
                            (5, 1, 3) => 5,
                            (6, 1, 3) => 6,
                            (3, 2, 2) => 7,
                            (7, 2, 4) => 8,
                            (6, 2, 4) => 9,
                            (_, _, _) => panic!(),
                        }
                })
                .sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(solve_part1(&parse(input)), 26);
        assert_eq!(solve_part2(&parse(input)), 61229);
    }
}
