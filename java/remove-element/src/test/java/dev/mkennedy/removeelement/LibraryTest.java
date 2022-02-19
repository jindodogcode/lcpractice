/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package dev.mkennedy.removeelement;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class LibraryTest {
    @Test
    void exampleOne() {
        int[] input = {3, 2, 2, 3};
        int val = 3;
        int result = Library.removeElement(input, val);

        assertEquals(2, result);
    }

    @Test
    void exampleTwo() {
        int[] input = {0, 1, 2, 2, 3, 0 , 4, 2};
        int val = 2;
        int result = Library.removeElement(input, val);

        assertEquals(5, result);
    }
}
