/*
 * Longest Common Prefix
 *
 * Write a function to find the longest commoon prefix string amongst an array of strings.
 * If there is no common prefix, return an empty string "".
 */
package dev.mkennedy.longestcommonprefix;

public class Library {
    public static String longestCommonPrefix(String[] strs) {
        if (strs.length == 0) return "";

        int lcpIndex = 0;
        boolean stop = false;

        for (int i = 0; i < strs[0].length(); ++i) {
            char c = strs[0].charAt(i);

            for (String s : strs) {
                if (i >= s.length() || c != s.charAt(i)) {
                    stop = true;
                    break;
                }
            }

            if (stop) { 
                break;
            } else {
                lcpIndex += 1;
            }
        }

        return strs[0].substring(0, lcpIndex);
    }
}
