""" 1. Two Sum

Given an array of integers nums and an integer target, rutun indices of the
two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not
use the same element twice.

You may return the answer in any order.

Constraints:
- 2 <= nums.length <= 10⁴
- -10⁹ <= nums[i] <= 10⁹
- -10⁹ <= target <= 10⁹
- Only one valid answer exists.

Follow up:
Can you com up with an algorithm that is less than O(n²) time complexity?
"""


class Solution:

    @staticmethod
    def twoSum(nums: 'list[int]', target: int) -> 'list[int]':
        value_index_map: dict[int, int] = dict()

        for i in range(len(nums)):
            dif = target - nums[i]

            if dif in value_index_map:
                return [value_index_map[dif], i]

            value_index_map[nums[i]] = i

        raise ValueError
