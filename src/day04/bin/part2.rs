//! # Advent of Code - Day 4 - Part Two

pub fn part2(numbers: &[i32], boards: &mut [Vec<Vec<i32>>]) -> i32 {
    let (win_num, win_board) = compute_game_with_last_board(numbers, boards);

    let sum: i32 = win_board.iter()
        .flat_map(|row| row.iter().map(|&value| if value == -1 { 0 } else { value } ))
        .sum();

    win_num * sum
}

pub fn compute_game_with_last_board(numbers: &[i32], boards: &[Vec<Vec<i32>>]) -> (i32, Vec<Vec<i32>>) {
    let num = numbers[0];

    let new_boards = mark_boards(num, boards);

    if new_boards.len() != 1 {
        let new_boards: Vec<Vec<Vec<i32>>> = new_boards.clone()
            .into_iter()
            .filter(|board| is_lost(board))
            .collect();

        return compute_game_with_last_board(&numbers[1..], &new_boards);
    }

    if new_boards.len() == 1 && is_lost(&new_boards[0]) == false {
        return (num, new_boards[0].clone());
    } else {
        return compute_game_with_last_board(&numbers[1..], &new_boards);
    }
}

fn mark_boards(num: i32, boards: &[Vec<Vec<i32>>]) -> Vec<Vec<Vec<i32>>> {
    let mut boards = boards.to_vec();
    for b in 0..boards.len() {
        for i in 0..5 {
            for j in 0..5 {
                if num == boards[b][i][j] {
                    boards[b][i][j] = -1;
                }
            }
        }
    }
    return boards;
}

fn is_lost(board: &[Vec<i32>]) -> bool {
    let board = board.clone();
    let t_board = transpose(&board);

    for i in 0..5 {
        if board[i] == vec![-1; 5] || t_board[i] == vec![-1; 5] {
            return false;
        }
    }

    true
}

fn transpose(mat: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = mat.len();
    let mut t_mat: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            t_mat[j][i] = mat[i][j];
        }
    }

    return t_mat;
}

#[cfg(test)]
mod day04 {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../tests.txt");
        let (numbers, mut boards) = crate::parse(input);

        assert_eq!(1924, part2(&numbers, &mut boards));
    }
}
