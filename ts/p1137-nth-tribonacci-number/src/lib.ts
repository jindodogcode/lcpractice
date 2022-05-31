/**
 * # 1137. N-th Tribonacci Number
 *
 * The Tribonacci sequence Tₙ is defined as follows:
 * T₀=0, T₁=1, T₂=1, and Tₙ₊₃=Tₙ+Tₙ₊₁+Tₙ₊₂ for n >= 0.
 * Given `n`, return the value of Tₙ.
 *
 * ## Constraints:
 *
 * - `0 <= n <= 37`
 * - The answer is guaranteed to fir within a 32-bit integer, ie. `answer <= 2³¹ - 1`.
 */

function tribonacci(n: number): number {
  if (n === 0) {
    return 0;
  } else if (n === 1 || n === 2) {
    return 1;
  }

  let t0 = 0;
  let t1 = 1;
  let t2 = 1;
  let t3 = 0;

  for (let i = 3; i <= n; ++i) {
    t3 = t0 + t1 + t2;
    t0 = t1;
    t1 = t2;
    t2 = t3;
  }

  return t3;
}

export { tribonacci };
