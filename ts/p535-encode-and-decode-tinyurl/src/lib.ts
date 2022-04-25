/**
 * # 535. Encode and Decode TinyURL
 *
 * TinyURL is a URL shortening service where you enter a URL such as `https://leetcode.com/problems/design-tinyurl` and it returns a short URL such as `http://tinyurl.com/4e9iAk`. Design a class to encode a URL and decode a tiny URL.
 *
 * There is no restriction how your encode/decode algorithm should work. You just need to ensure that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original URL.
 *
 * Implement the `Solution` class:
 * - `Solution()` Initializes the object of the system.
 * - `String encode(String longUrl)` Returns a tiny URL for the given `longUrl`.
 * - `String decode(String shortUrl)` Returns the original long URL for the given `shortUrl`. It is guaranteed that the given `shortUrl` was encoded by the same object.
 *
 * ## Constraints:
 *
 * - `1 <= url.length <= 10⁴`
 * - `url` is guranteed to be a valid URL.
 */

let count = 0;
let map: Map<string, string> = new Map();

function encode(longUrl: string): string {
  const digits = new Array<string>();
  let n = count;

  for (let i = 0; i < 8; ++i) {
    const digit = n % 62;
    n /= 62;
    digits.push(convertDigit(digit));
  }

  count += 1;
  const tiny = digits.reverse().join("");
  map.set(tiny, longUrl);

  return tiny;
}

function decode(shortUrl: string): string {
  const long = map.get(shortUrl);

  if (long) {
    return long;
  } else {
    return "";
  }
}

function convertDigit(digit: number): string {
  if (digit >= 0 && digit < 26) {
    return String.fromCharCode(digit + 97);
  } else if (digit >= 26 && digit < 52) {
    return String.fromCharCode(digit + 15);
  } else if (digit >= 52 && digit < 62) {
    return String.fromCharCode(digit - 4);
  } else {
    return "-";
  }
}

export { encode, decode };
