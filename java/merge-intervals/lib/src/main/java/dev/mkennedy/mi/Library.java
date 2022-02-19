/**
 * <h1>Merge Intervals</h1> Given an array of <code>intervals</code> where
 * <code>intervals[i] = [startᵢ, endᵢ]</code>, merge all overlapping intervals,
 * and return <em>an array of the non-overlapping intervals that cover all the
 * interval in the input</em>.
 * <h2>Contraints:</h2>
 * <ul>
 * <li>1 &lt;= intervals.length &lt;= 10⁴</li>
 * <li>intervals[i].length == 2</li>
 * <li>0 &lt;= startᵢ &lt;= endᵢ &lt;= 10⁴</li>
 * </ul>
 *
 * @author Michael Kennedy
 * @version 1.0
 */
package dev.mkennedy.mi;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Library {
    public boolean someLibraryMethod() {
        return true;
    }

    public static int[][] merge(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> {
            return a[0] - b[0];
        });
        List<int[]> merged = new ArrayList<>();
        int lastIndex = merged.size() - 1;

        for (int[] interval : intervals) {
            if (merged.isEmpty() || merged.get(lastIndex)[1] < interval[0]) {
                merged.add(interval);
                lastIndex = merged.size() - 1;
            } else {
                if (merged.get(lastIndex)[1] < interval[1]) {
                    merged.get(lastIndex)[1] = interval[1];
                }
            }
        }

        return merged.toArray(new int[merged.size()][]);
    }
}
