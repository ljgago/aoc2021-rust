//! # Advent of Code - Day 4 - Part Two

pub fn bingo_score(numbers: &[i32], boards: &mut [Vec<Vec<i32>>]) -> Result<i32, &'static str> {
    let (win_num, win_board) = compute_game_with_last_board(numbers, boards);

    let sum: i32 = win_board.iter()
        .flat_map(|row| row.iter().map(|&value| if value == -1 { 0 } else { value } ))
        .sum();

    Ok(win_num * sum)
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
    fn test_bingo_score() {
        let input = r#"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
            2  0 12  3  7
            "#;
        let (numbers, mut boards) = crate::parse(input);
        assert_eq!(Ok(1924), bingo_score(&numbers, &mut boards));
    }
}
