/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package dev.mkennedy.coursescheduleii;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

class LibraryTest {
    @Test
    void testExampleOne() {
        int numCourses = 2;
        int[][] prereqs = {{1, 0}};
        List<Integer> result =
                toIntegerList(Library.findOrder(numCourses, prereqs));
        List<Integer> ans = List.of(0, 1);

        assertEquals(ans, result);

    }

    @Test
    void testExampleTwo() {
        int numCourses = 4;
        int[][] prereqs = {{1, 0}, {2, 0}, {3, 1}, {3, 2}};
        List<Integer> result =
                toIntegerList(Library.findOrder(numCourses, prereqs));
        List<List<Integer>> ans =
                List.of(List.of(0, 1, 2, 3), List.of(0, 2, 1, 3));

        assertTrue(ans.contains(result));
    }

    @Test
    void testExampleThree() {
        int numCourses = 1;
        int[][] prereqs = {};
        List<Integer> result =
                toIntegerList(Library.findOrder(numCourses, prereqs));
        List<Integer> ans = List.of(0);

        assertEquals(ans, result);
    }

    static List<Integer> toIntegerList(int[] arr) {
        return Arrays.stream(arr).boxed().collect(Collectors.toList());
    }
}
