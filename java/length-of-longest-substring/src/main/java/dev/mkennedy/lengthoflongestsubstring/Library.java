/*
 * Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 */

package dev.mkennedy.lengthoflongestsubstring;

import java.util.HashMap;
import java.util.Map;

public class Library {
    public static int lengthOfLongestSubstring(String s) {
        int max = 0;
        Map<Character, Integer> indices = new HashMap<>(s.length());

        for (int i = 0, j = 0; j < s.length(); ++j) {
            char c = s.charAt(j);

            if (indices.containsKey(c)) {
                i = Math.max(i, indices.get(c));
            }

            max = Math.max(max, j - i + 1);
            indices.put(c, j + 1);
        }

        return max;
    }
}
