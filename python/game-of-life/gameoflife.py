""" 289. Game of Life

According to Wikipedia's article: "The Game of Life, also known simply as Life,
is a cellular automation devisded by the British mathematician John Horton
Conway in 1970."

The board is made up of an m x n grid of cells, where each cell has an initial
state: live (represented by a 1) or dad (represented by a 0). Each cell
interacts with its eight neighbors (horizontal, vertical, diagonal) using the
following four rules (taken from the above Wikipedia article):
    1. Any live cell with fewer than two live neighbors dies as if caused by
       under-population.
    2. Any live cell with two or three live neighbors lives on to the next
       generation.
    3. Any live cell with more than three live neighbors dies, as if by over-
       population.
    4. Any dead cell with exactly three live neighbors becomes a live cell, as
       if by reproduction.

The next state is created by applying the above rules simultaneously to every
cell in the current state, where births and deaths occur simultaneously. Given
the curren state of the m x n grid board, return the next state.

Constraints:
- m == board.length
- n == board[i].length
- 1 <= m, n <= 25
- board[i][j] is 0 or 1
"""

ALIVE = 1
DEAD = 0


class Solution:

    @staticmethod
    def gameOfLife(board: 'list[list[int]]') -> None:
        prev_row = None

        for row in range(len(board)):
            cur_row = []

            for col in range(len(board[row])):
                cur_row.append(Solution.calcCell(board, row, col))

            if prev_row:
                board[row-1] = prev_row

            prev_row = cur_row

        if prev_row:
            board[-1] = prev_row

    @staticmethod
    def calcCell(board: 'list[list[int]]', row: int, col: int) -> int:
        neighbor_count = 0

        for row_offset in (-1, 0, 1):
            for col_offset in (-1, 0, 1):
                if row_offset == 0 and col_offset == 0:
                    continue

                n_row = row + row_offset
                if n_row < 0 or n_row >= len(board):
                    continue

                n_col = col + col_offset
                if n_col < 0 or n_col >= len(board[n_row]):
                    continue

                if board[n_row][n_col] == ALIVE:
                    neighbor_count += 1

        if board[row][col] == ALIVE and neighbor_count in (2, 3):
            return ALIVE
        if board[row][col] == DEAD and neighbor_count == 3:
            return ALIVE
        return DEAD
