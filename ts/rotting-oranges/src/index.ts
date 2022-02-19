/**
 * # 994. Rotting Oranges
 *
 * You are given an `m x n` `grid` where each cell can have on of three values:
 * - `0` representing an empty cell,
 * - `1` representing a fresh orange, or
 * - `2` representing a rotten orange.
 *
 * Every minute, any fresh orange that is **4-directionally adjacent** to a rotten orange becomes rotten.
 *
 * Return _the minimum number of minutes that must elapse until no cell has a fresh orange_. If _this is impossible, return_ `-1`.
 *
 * ## Constraints:
 * - `m == grid.length`
 * - `n == grid[i].length`
 * - `1 <= m, n <= 10`
 * - `grid[i][j]` is `0`, `1`, or `2`.
 */
import { Queue } from "@datastructures-js/queue";

export function orangesRotting(grid: number[][]): number {
  const ROWS = grid.length;
  const COLS = grid[0].length;

  let minutes = -1;
  const queue: Queue<[row: number, col: number]> = new Queue();
  let freshCount: number = 0;
  for (let row = 0; row < grid.length; ++row) {
    for (let col = 0; col < grid[0].length; ++col) {
      let value = grid[row][col];

      if (value === 1) {
        freshCount += 1;
      } else if (value == 2) {
        queue.enqueue([row, col]);
      }
    }
  }
  queue.enqueue([-1, -1]);

  const offsets: [row: number, col: number][] = [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1]
  ];

  while (!queue.isEmpty()) {
    let rotten = queue.dequeue();
    if (!rotten) {
      throw Error("undefined rotten orange");
    }

    let [row, col] = rotten;
    if (row === -1) {
      minutes += 1;
      if (!queue.isEmpty()) {
        queue.enqueue([-1, -1]);
      }
    } else {
      for (let [oRow, oCol] of offsets) {
        const nRow = row + oRow;
        const nCol = col + oCol;

        if (nRow >= 0 && nRow < ROWS && nCol >= 0 && nCol < COLS) {
          if (grid[nRow][nCol] === 1) {
            grid[nRow][nCol] = 2;
            freshCount -= 1;
            queue.enqueue([nRow, nCol]);
          }
        }
      }
    }
  }

  return freshCount === 0 ? minutes : -1;
}
