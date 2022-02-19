//! # 289. Game of Life
//!
//! According to Wikipedia's article: "The __Game of Life__, also known simply as __Life__, is a
//! cellular automation devised by the British mathematician Jon Horton Conway in 1970."
//!
//! The board is made up of an `m x n` grid of cells, where each cell has an initial state:
//! __live__ (represented by a 1) or __dead__ (represented by a 0). Each cell interacts with its
//! eight neighbors (horizontal, vertical, diagonal) using the follow four rules (take from the
//! above Wikipedia article):
//! 1. Any live cell with fewer than two live neighbors dies as if caused by under population.
//! 2. Any live cell with two or three live neighbors lives on to the next generation.
//! 3. Any live cell with more than three live neighbors dies, as if by over-population.
//! 4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
//!
//! The next state is created by applying the above rules simultaneously to every cell in the
//! current state, where births and deaths occur simultaneously. Given the current state of the `m
//! x n` grid `board`, return _the next state_.
//!
//! ## Constraints:
//! - `m == board.length`
//! - `n == board[i].length`
//! - `1 <= m, n <= 25`
//! - `board[i][j]` is `0` or `1`.
//!
//! ## Follow up:
//! - Could you solve it in-place? Remember that the board needs to be update simultaneously: You
//! cannot update some cells first and then use their update values to update other cells.
//! - In this question, we represent the board using a 2D array. In principle, the board is
//! infinite, which would case problems when the active area enchroaches upon the boarder of the
//! array(.i.e., live cells reach the border). How would you address these problems?

struct Solution;

const ALIVE_CELL: i32 = 1;
const DEAD_CELL: i32 = 0;

#[allow(clippy::if_same_then_else)]
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut prev_row: Option<Vec<i32>> = None;

        for row in 0..board.len() {
            let mut new_row: Vec<i32> = vec![0; board[0].len()];
            for col in 0..board[row].len() {
                let updated_cell = Solution::calc_cell(board, row, col);
                new_row[col] = updated_cell;
            }

            if let Some(p_row) = prev_row {
                board[row - 1] = p_row;
            }

            prev_row = Some(new_row);
        }

        let board_len = board.len();
        board[board_len - 1] = prev_row.unwrap();
    }

    fn calc_cell(board: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        let mut neighbor_count = 0;

        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let neighbor_row = row as i32 + row_offset;
                if neighbor_row < 0 || neighbor_row >= board.len() as i32 {
                    continue;
                }
                let neighbor_row = neighbor_row as usize;

                let neighbor_col = col as i32 + col_offset;
                if neighbor_col < 0 || neighbor_col >= board[neighbor_row].len() as i32 {
                    continue;
                }
                let neighbor_col = neighbor_col as usize;

                if board[neighbor_row][neighbor_col] == ALIVE_CELL {
                    neighbor_count += 1;
                }
            }
        }

        if board[row][col] == ALIVE_CELL && (2..=3).contains(&neighbor_count) {
            ALIVE_CELL
        } else if board[row][col] == DEAD_CELL && neighbor_count == 3 {
            ALIVE_CELL
        } else {
            DEAD_CELL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut input = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut input);
        let expected = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];

        assert_eq!(expected, input);
    }

    #[test]
    fn example_two() {
        let mut input = vec![vec![1, 1], vec![1, 0]];
        Solution::game_of_life(&mut input);
        let expected = vec![vec![1, 1], vec![1, 1]];

        assert_eq!(expected, input);
    }
}
