//! # 535. Enode and Decode TinyURL
//!
//! TinyURL is a URL shortening service where you enter a URL such as
//! `https://leetcode.com/problems/design-tinyurl` and it returns a short URL such as `
//! http://tinyurl.com/439iAk`. Design a class to encode a URL and decode a tiny URL.
//!
//! There is no restriction on how your encode/decode algorithm should work. You just need to
//! ensure that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original
//! URL.
//!
//! Implement the `Solution` class:
//! - `Solution()` Initializes the object of the system.
//! - `String encode(String longUrl)` Returns a tiny URL for the given `longUrl`.
//! - `String decode(stwString shortUrl)` Returns the original long URL for the given `shortUrl`.
//! It is guaranteed that the given `shortUrl` was encoded by the same object.
//!
//! ## Constraints:
//!
//! - `1 <= url.length <= 10⁴`
//! - `url` is guranteed to be a valid URL.

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Codec {
    count: usize,
    map: HashMap<String, String>,
}

impl Codec {
    const SHORT_SIZE: usize = 8;
    const SHORT_BASE: usize = 62;

    pub fn new() -> Self {
        Codec {
            count: 0,
            map: HashMap::new(),
        }
    }

    pub fn encode(&mut self, long_url: String) -> String {
        let mut digits = Vec::with_capacity(Self::SHORT_SIZE);
        let mut count = self.count;
        for _i in 0..8 {
            let dig = Self::convert_digit((count % Self::SHORT_BASE) as u8);
            count /= Self::SHORT_BASE;
            digits.push(dig);
        }

        let short_url: String = digits.iter().rev().collect();
        self.count += 1;
        self.map.insert(short_url.clone(), long_url);

        short_url
    }

    pub fn decode(&self, short_url: String) -> String {
        if let Some(long_url) = self.map.get(&short_url) {
            long_url.clone()
        } else {
            "".to_owned()
        }
    }

    fn convert_digit(digit: u8) -> char {
        match digit {
            0..=25 => (digit + 97) as char,
            26..=51 => (digit + 15) as char,
            52..=61 => (digit - 4) as char,
            _ => '-',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = ["https://leetcode.com/problems/design-tinyurl"];
        let expected = 0;

        run_test(&input, expected);
    }

    #[test]
    fn multiple_inputs() {
        let input = [
            "https://leetcode.com/problems/design-tinyurl",
            "https://twitter.com",
            "https://google.com",
        ];
        let expected = 1;

        run_test(&input, expected);
    }

    fn run_test(input: &[&str], expected: usize) {
        let mut codec = Codec::new();
        let tiny = input
            .iter()
            .map(|&i| codec.encode(i.to_owned()))
            .collect::<Vec<String>>();
        let output = codec.decode(tiny[expected].clone());

        assert_eq!(output, input[expected]);
    }
}
