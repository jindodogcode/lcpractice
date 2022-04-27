/**
 * # 746. Min Cost Climbing Stairs
 *
 * You are given an integer array `cost` where `cost[i]` is the cost of `iᵗʰ` step on a staircase. Once you pay the cost, you can either climb one or two steps.
 *
 * You can either start from the step with index `0`, or the step with index `1`.
 *
 * Return *the minimum cost to reach the top of the floor*.
 *
 * ## Constraints:
 *
 * - `2 <= cost.length <= 1000`
 * - `0 <= cost[i] <= 999`
 */

function minCostClimbingStairs(cost: number[]): number {
  const n = cost.length;

  if (n === 2) {
    return Math.min(cost[0], cost[1]);
  }

  const costTable = new Array<number>(n);
  costTable[0] = cost[0];
  costTable[1] = cost[1];

  for (let i = 2; i < n; ++i) {
    const value = cost[i];
    costTable[i] = Math.min(costTable[i - 1] + value, costTable[i - 2] + value);
  }

  return Math.min(costTable[n - 1], costTable[n - 2]);
}

export { minCostClimbingStairs };
