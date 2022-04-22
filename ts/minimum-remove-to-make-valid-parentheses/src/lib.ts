/**
 * # 1249. Minimum Remove to Make Valid Parentheses
 *
 * Given a string s of `'('`, `')'` and lowercase English charecters.
 *
 * Your task is to remove the minimum number of parentheses(`'('` or `')'`, in any positions) so that the resulting *parentheses string* is valid and return **any** valid string.
 *
 * Formally, a *parentheses string* is valid if and only if:
 * - It is the empty string, contains only lowercase charecters, or
 * - It can be written as `AB` (`A` concatenated with `B`), where `A` and `B` are valid string, or
 * - It can be written as `(A)`, where `A` is a valid string.
 *
 * ## Constraints:
 *
 * - `1 <= s.length <= 10⁵`
 * - `s[i]` is either `'('`, `')'` or lowercase English letter.
 */

function minRemoveToMakeValid(s: string): string {
  const chars = [...s];
  let count = 0;
  const remove = new Set<number>();

  for (const [i, c] of chars.entries()) {
    if (c === "(") {
      count += 1;
    } else if (c === ")") {
      if (count > 0) {
        count -= 1;
      } else {
        remove.add(i);
      }
    }
  }

  if (count > 0) {
    for (let i = chars.length - 1; i >= 0; --i) {
      let c = s.charAt(i);
      if (c === "(") {
        count -= 1;
        remove.add(i);
      }
      if (count === 0) {
        break;
      }
    }
  }

  const output = new Array<string>();
  for (const [i, c] of chars.entries()) {
    if (!remove.has(i)) {
      output.push(c);
    }
  }

  return output.join("");
}

export { minRemoveToMakeValid };
