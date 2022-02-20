/**
 * # 1. Two Sum
 *
 * Given an array of integers `nums` and an integer `target`, return *indicies of the two numbers such that they add up to `target`*.
 *
 * You may assume that each input would have **exactly one solution**, and you may not use the *same* element twice.
 *
 * You can return the answer in any order.
 *
 * ## Constraints:
 *
 * - `2 <= nums.length <= 10⁴`
 * - `10⁹ <= nums[i] <= 10⁹`
 * - `10⁹ <= target <= 10⁹`
 * - **Only one valid answer exists.**
 */

export function twoSum(nums: number[], target: number): number[] {
  const remMap: Map<number, number> = new Map();

  for (let i = 0; i < nums.length; ++i) {
    let val = nums[i];
    let remIndex = remMap.get(val);
    if (typeof remIndex === "number") {
      return [remIndex, i];
    }
    remMap.set(target - val, i);
  }

  console.log(remMap);

  return [];
}
