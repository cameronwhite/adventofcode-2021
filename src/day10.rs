#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn end_pair(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

fn check_corrupted(s: &str) -> Option<char> {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(end_pair(c)),
            ')' | ']' | '}' | '>' => {
                if *stack.last().unwrap() == c {
                    stack.pop();
                } else {
                    return Some(c);
                }
            }
            _ => panic!(),
        }
    }

    None
}

#[aoc(day10, part1)]
fn solve_part1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|s| match check_corrupted(s) {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        })
        .sum()
}

fn find_incomplete(s: &str) -> Option<usize> {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(end_pair(c)),
            ')' | ']' | '}' | '>' => {
                if *stack.last().unwrap() == c {
                    stack.pop();
                } else {
                    return None; // corrupted
                }
            }
            _ => panic!(),
        }
    }

    // Leftover required characters.
    stack.reverse();

    Some(stack.iter().fold(0, |total, c| {
        total * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!(),
            }
    }))
}

#[aoc(day10, part2)]
fn solve_part2(lines: &Vec<String>) -> usize {
    let mut scores: Vec<usize> = lines.iter().filter_map(|s| find_incomplete(s)).collect();

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(solve_part1(&parse(input)), 26397);
        assert_eq!(solve_part2(&parse(input)), 288957);
    }
}
