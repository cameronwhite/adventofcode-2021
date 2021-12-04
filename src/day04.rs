type Board = Vec<Vec<i32>>;

#[aoc_generator(day4)]
fn parse(input: &str) -> (Vec<i32>, Vec<Board>) {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let boards = lines
        .filter(|s| s.len() != 0)
        .map(|s| {
            s.split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Board>()
        .chunks(5)
        .map(|b| b.to_vec())
        .collect();

    (numbers, boards)
}

fn update_board(board: &mut Board, num: i32) {
    for line in board {
        for entry in line {
            if *entry == num {
                *entry = -1;
            }
        }
    }
}

fn check_board(board: &Board) -> bool {
    // Rows
    for line in board {
        if line.iter().all(|entry| *entry == -1) {
            return true;
        }
    }

    // Columns
    for i in 0..5 {
        if board.iter().all(|line| line[i] < 0) {
            return true;
        }
    }

    return false;
}

fn board_score(board: &Board) -> i32 {
    board
        .iter()
        .map(|line| line.iter().filter(|&entry| *entry >= 0).sum::<i32>())
        .sum()
}

#[aoc(day4, part1)]
fn solve_part1(input: &(Vec<i32>, Vec<Board>)) -> i32 {
    let (numbers, boards) = input;
    let mut boards = boards.clone();

    for num in numbers {
        for board in &mut boards {
            update_board(board, *num);
        }

        for board in &boards {
            if check_board(board) {
                return board_score(board) * num;
            }
        }
    }

    -1
}

#[aoc(day4, part2)]
fn solve_part2(input: &(Vec<i32>, Vec<Board>)) -> i32 {
    let (numbers, boards) = input;
    let mut boards = boards.clone();
    let mut last_score = -1;

    for num in numbers {
        for board in &mut boards {
            update_board(board, *num);
        }

        for board in &boards {
            if check_board(board) {
                last_score = board_score(board) * num;
            }
        }

        boards.retain(|board| !check_board(board));
    }

    last_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(solve_part1(&parse(input)), 4512);
        assert_eq!(solve_part2(&parse(input)), 1924);
    }
}
