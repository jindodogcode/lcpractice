/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package dev.mkennedy.gol;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.Arrays;

class LibraryTest {

    @Test
    void exampleOne() {
        int[][] input = { { 0, 1, 0 }, { 0, 0, 1 }, { 1, 1, 1 }, { 0, 0, 0 } };
        Library.gameOfLife(input);
        int[][] expected = { { 0, 0, 0 }, { 1, 0, 1 }, { 0, 1, 1 }, { 0, 1, 0 } };

        for (int i = 0; i < input.length; ++i) {
            assertTrue(Arrays.equals(expected[i], input[i]));
        }
    }

    @Test
    void exampleTwo() {
        int[][] input = { { 1, 1 }, { 1, 0 } };
        Library.gameOfLife(input);
        int[][] expected = { { 1, 1 }, { 1, 1 } };

        for (int i = 0; i < input.length; ++i) {
            assertTrue(Arrays.equals(expected[i], input[i]));
        }
    }
}
