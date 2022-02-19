/**
 * # 1. Two Sum
 *
 * Given an array of integers `nums` and an integer `target`, return _indices
 * of the two numbers such that they add up to_ `target`.
 *
 * You may assume that each input would have __exactly one solution__, and you
 * may not use the _same_ element twice.
 *
 * You can return the answer in any order.
 *
 * ## Constraints:
 * - `2 <= nums.length <= 10⁴`
 * - `-10⁹ <= nums[i] <= 10⁹`
 * - `-10⁹ <= target <= 10⁹`
 * - __Only one valid answer exists.__
 *
 * ## Follow-up:
 * Can you come up with an algorithm that is less than `O(n²)` time complexity?
 */

/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
const twoSum = function (nums, target) {
  const map = {};
  for (let i = 0; i < nums.length; ++i) {
    const dif = target - nums[i];
    if (dif in map) {
      return [map[dif], i];
    }
    map[nums[i]] = i;
  }

  throw new Error("No solution");
};

export { twoSum };
