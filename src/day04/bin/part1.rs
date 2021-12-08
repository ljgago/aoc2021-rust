//! # Advent of Code - Day 4 - Part One

pub fn part1(numbers: &[i32], boards: &mut [Vec<Vec<i32>>]) -> i32 {
    let (win_num, win_board_index) = compute_game(numbers, boards);

    let board = boards[win_board_index].clone();

    let sum: i32 = board.iter()
        .flat_map(|row| row.iter().map(|&value| if value == -1 { 0 } else { value } ))
        .sum();

    win_num * sum
}

pub fn compute_game(numbers: &[i32], boards: &mut [Vec<Vec<i32>>]) -> (i32, usize) {
    let num = numbers[0];

    mark_boards(num, boards);

    let board_index = get_winner_board_index(boards);

    if let Some(index) = board_index {
        return (num, index);
    } else {
        return compute_game(&numbers[1..], boards);
    }
}

fn mark_boards(num: i32, boards: &mut [Vec<Vec<i32>>]) {
    for b in 0..boards.len() {
        for i in 0..5 {
            for j in 0..5 {
                if num == boards[b][i][j] {
                    boards[b][i][j] = -1;
                }
            }
        }
    }
    return;
}

fn get_winner_board_index(boards: &[Vec<Vec<i32>>]) -> Option<usize> {
     for b in 0..boards.len() {
         let board = boards[b].clone();
         let t_board = transpose(&boards[b]);

         for i in 0..5 {
             if board[i] == vec![-1; 5] || t_board[i] == vec![-1; 5] {
                 return Some(b);
             }
         }
     }
    None
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
    fn test_part1() {
        let input = include_str!("../tests.txt");
        let (numbers, mut boards) = crate::parse(input);

        assert_eq!(4512, part1(&numbers, &mut boards));
    }
}
