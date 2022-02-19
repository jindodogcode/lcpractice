/** 
 * <h1>289. Game of Life</h1>
 *
 * According to Wikipedia's article: "The <strong>Game of Life</strong>, also known simply as 
 * <strong>Life</strong>, is a cellular automaton devised by British mathematician John Horton 
 * Conway in 1970."
 *
 * The board is made up of an <code>m x n</code> grid of cells, where each cell has an initial 
 * state: <strong>live</strong> (represented by a 1) or <strong>dead</strong> (represented by a 0).
 * Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the 
 * following four rules (taken from the above Wikipedia article):
 * <ol>
 *  <li>Any live cell with fewer than two live neighbors dies as if caused by under-population.</li>
 *  <li>Any live cell with two or three live neighbors lives on to the next generation.</li>
 *  <li>Any live cell with more than three live neighbors dies, as if by over-population.</li>
 *  <li>Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.</li>
 * </ol>
 * The next state is created by applying the above rules simultaneously to every cell in the
 * current state, where births and deaths occur simultaneously. Given the current state of the 
 * <code>m x n</code> grid <code>board</code>, return <em>the next state</em>.
 * */

package dev.mkennedy.gol;

import java.util.stream.IntStream;

public class Library {
    private static int ALIVE = 1;
    private static int DEAD = 0;

    public static void gameOfLife(int[][] board) {
        int[] prevRow = null;

        for (int row = 0; row < board.length; ++row) {
            int[] curRow = new int[board[row].length];

            for (int col = 0; col < board[row].length; ++col) {
                int cell = calcCell(board, row, col);
                curRow[col] = cell;
            }

            if (prevRow != null) {
                board[row - 1] = prevRow;
            }

            prevRow = curRow;
        }

        board[board.length - 1] = prevRow;
    }

    private static int calcCell(int[][] board, int row, int col) {
        int neighborCount = 0;

        for (int rowOffset : IntStream.rangeClosed(-1, 1).toArray()) {
            for (int colOffset : IntStream.rangeClosed(-1, 1).toArray()) {
                if (rowOffset == 0 && colOffset == 0) {
                    continue;
                }

                int nRow = row + rowOffset;
                if (nRow < 0 || nRow >= board.length) {
                    continue;
                }

                int nCol = col + colOffset;
                if (nCol < 0 || nCol >= board[row].length) {
                    continue;
                }

                if (board[nRow][nCol] == ALIVE) {
                    neighborCount += 1;
                }
            }
        }

        if (board[row][col] == ALIVE && neighborCount >= 2 && neighborCount <= 3) {
            return ALIVE;
        }
        if (board[row][col] == DEAD && neighborCount == 3) {
            return ALIVE;
        }

        return DEAD;
    }
}
