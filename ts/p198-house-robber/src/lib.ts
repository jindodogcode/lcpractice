/**
 * # 198. House Robber
 *
 * You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security system connected and **it will automatically contact the polic if two adjacent houses were broken into on the same night**.
 *
 * Given an integer array `nums` representing the amount of money of each house, return *the maximum amount of money you can rob tonight* **without alerting the police**.
 *
 * ## Constraints:
 *
 * - `1 <= nums.length <= 100`
 * - `0 <= nums[i] <= 400`
 */

function rob(nums: number[]): number {
  const n = nums.length;
  if (n === 1) {
    return nums[0];
  }

  const values = new Array<number>(n + 1).fill(0);
  values[n - 1] = nums[n - 1];

  for (let i = n - 2; i >= 0; --i) {
    values[i] = Math.max(values[i + 1], values[i + 2] + nums[i]);
  }

  return values[0];
}

export { rob };
