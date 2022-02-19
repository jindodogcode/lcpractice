/**
 * # 7. Reverse Integer
 *
 * Given a signed 32-bit integer `x`, return `x` with _its digits reversed_. If reversing `x`
 * causes the value to go outside the signed 32-bit integer range `[-2³¹, 2³¹ - 1]`, then return
 * `0`.
 *
 * **Assume the environment does not allow you to store 64-bit integers (signed or unsigned).**
 *
 * ## Constraints:
 * - `-2³¹ <= x <= 2³¹ - 1`
 */
function reverse(x) {
    let num = x;
    let rev = 0;
    const sign = num >= 0 ? 1 : -1;
    num = Math.abs(num);
    while (num > 0) {
        let digit = num % 10;
        rev = rev * 10 + digit;
        num = Math.trunc(num / 10);
    }
    if (rev < Math.pow(-2, 31) || rev > Math.pow(2, 31) - 1) {
        return 0;
    }
    return rev * sign;
}
;
export { reverse };
