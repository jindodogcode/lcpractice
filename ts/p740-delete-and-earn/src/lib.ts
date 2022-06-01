/**
 * # 740. Delete and Earn
 *
 * You are given an integer array `nums`. You want to maximize the number of points you get by performing the following operation any number of times:
 * - Pick any `nums[i]` and delete it to earn `nums[i]` points. Afterwards, you must delete **every** element equal to `nums[i] - 1` and **every** element equal to `nums[i] + 1`.
 *
 * *Return the **maximum number of points** you can earn by applying the abover operation some number of times*.
 *
 * ## Constraints:
 *
 * - `1 <= nums.length <= 2*10⁴`
 * - `1 <= nums[i] <= 10⁴`
 */

function deleteAndEarn(nums: number[]): number {
  const points: Map<number, number> = new Map();

  for (let n of nums) {
    let p = points.get(n);
    if (p) {
      points.set(n, p + n);
    } else {
      points.set(n, n);
    }
  }

  const pointsSorted = Array.from(points.entries());
  pointsSorted.sort((a, b) => a[0] - b[0]);

  let oneBack: [number, number] = [0, 0];
  let twoBack: [number, number] = [0, 0];

  for (let t of pointsSorted) {
    const temp = oneBack;

    if (t[0] - 1 === oneBack[0]) {
      oneBack = [t[0], Math.max(t[1] + twoBack[1], oneBack[1])];
    } else {
      oneBack = [t[0], t[1] + oneBack[1]];
    }

    twoBack = temp;
  }

  return oneBack[1];
}

export { deleteAndEarn };
