/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package dev.mkennedy.containerwithmostwater;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class LibraryTest {
    @Test
    void exampleOne() {
        int[] input = {1, 8, 6, 2, 5, 4, 8, 3, 7};
        int result = Library.maxArea(input);

        assertEquals(49, result);
    }
}
