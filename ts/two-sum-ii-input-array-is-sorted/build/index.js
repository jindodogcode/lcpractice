/**
 * # Two Sum II - Input array is sorted
 *
 * Given a **1-indexed** array of integers `numbers` that is already **sorted in non-decreasing order**, find two numbers such that they add up to a specific `target` number. Let these two numbers be `numbers[index₁]` and `numbers[index₂]` where `1 <= fist < second <= numbers.length`.
 *
 * Return _the indices of the two numbers_, `index₁` and `index₂`, as an _integer array_ `[index₁, index₂]` _of length 2_.
 *
 * The tests are generated such that there is **exactly one solution**. You **may not** use the same element twice.
 *
 * ## Constraints:
 * - `2 <= numbers.length <= 3 * 10⁴`
 * - `-1000 <= numbers[i] <= 1000`
 * - `numbers` is soreted in **non-decreasing order**.
 * - `-1000 <= target <= 1000`
 * - The tests are generated such that there is **exactly one solution**.
 */
function twoSum(numbers, target) {
    let left = 0;
    let right = numbers.length - 1;
    while (left < right) {
        const sum = numbers[left] + numbers[right];
        if (sum < target) {
            left += 1;
        }
        else if (sum > target) {
            right -= 1;
        }
        else {
            return [left + 1, right + 1];
        }
    }
    throw Error("No solution");
}
export { twoSum };
//# sourceMappingURL=index.js.map