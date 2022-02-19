/**
 * # 289. Game of Life
 *
 * According to Wikipedia's article: "The __Game of Life__, also known simply
 * as __Life__, is a cellular automaton devised by the British mathematician 
 * John Horton Conway in 1970."
 *
 * The board is made up of an `m x n` grid of cells, where each cell has an 
 * initial state: __live__ (represented by `1`) or __dead__ (represented by a
 * `0`). Each cell interacts with its eight neighbors (horizontal, vertical,
 * diagonal) using the following four rules (take from the above Wikipedia 
 * article):
 * 1. Any live cell with fewer than two live neighbors dies as if caused by
 * under-population
 * 2. Any live cell with two or three live neighbors lives on to the next 
 * generation.
 * 3. Any live cell with more than three live neighbors dies, as if by over-
 * population.
 * 4. Any dead cell with exactly three live neighbors becomes a live cell, as 
 * if by reproduction.
 *
 * The next state is created by applying the above rules simultaneously to 
 * every cell in the current state, where births and deaths occur 
 * simultaneously. Given the current state of the `m x n` grid `board`, return
 * _the next state_.
 *
 * ## Constraints:
 * - `m == board.length`
 * - `n == board[i].length`
 * - `1 <= m, n <= 25
 * - `board[i][j]` is `0` or `1`.
 *
 * ## Follow up:
 * - Could you solve it in-place? Remember that the board needs to be updated
 * simultaneously: You cannot update some cells first and then use their 
 * updated values to update other cells.
 * - In this question, we represent the board using a 2D array. In principle,
 * the board is infinite, which would cause problems when the active area
 * encroaches upon the border of the array (i.e. live cells reach the boarder).
 * How would you addres these problems?
 */

const ALIVE = 1;
const DEAD = 0;

function gameOfLife(board: number[][]): void {
    let prevRow: number[] | undefined = undefined; 

    for (let row = 0; row < board.length; ++row) {
        let curRow: number[] = [];

        for (let col = 0; col < board[row].length; ++col) {
            curRow.push(calcCell(board, row, col));
        }

        if (typeof prevRow === "object") {
            board[row - 1] = prevRow;
        }

        prevRow = curRow;
    }

    if (typeof prevRow === "object") {
        board[board.length - 1] = prevRow;
    }
}

function calcCell(board: number[][], row: number, col: number): number {
    let neighborCount = 0;

    for (let rowOffset of [-1, 0, 1]) {
        for (let colOffset of [-1, 0, 1]) {
            if (rowOffset === 0 && colOffset === 0) {
                continue;
            }

            const nRow = row + rowOffset;
            if (nRow < 0 || nRow >= board.length) {
                continue;
            }

            const nCol = col + colOffset;
            if (nCol < 0 || nCol >= board[nRow].length) {
                continue;
            }

            if (board[nRow][nCol] === ALIVE) {
                neighborCount += 1;
            }
        }
    }

    if (board[row][col] === ALIVE && [2, 3].includes(neighborCount)) {
        return ALIVE;
    }
    if (board[row][col] === DEAD && neighborCount === 3) {
        return ALIVE;
    }

    return DEAD;
}

export { gameOfLife };
