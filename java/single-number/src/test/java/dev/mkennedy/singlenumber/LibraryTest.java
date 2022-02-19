/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package dev.mkennedy.singlenumber;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class LibraryTest {
    @Test
    void exampleOne() {
        int[] input = {2, 2, 1};
        int result = Library.singleNumber(input);

        assertEquals(1, result);
    }

    @Test
    void exampleTwo() {
        int[] input = {4, 1, 2, 1, 2};
        int result = Library.singleNumber(input);

        assertEquals(4, result);
    }
}
