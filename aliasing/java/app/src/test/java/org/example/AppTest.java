/*
 * This source file was generated by the Gradle 'init' task
 */
package org.example;

import org.junit.Test;

import static org.junit.Assert.*;

public class AppTest {
    @Test
    public void testAliasing_pass() {
        int[] input = {12};
        int[] output = {12};
        App app = new App();
        app.compute(input, output);
        assertArrayEquals(new int[]{2}, output);
    }

    @Test
    public void testAliasing_fail() {
        int[] input = {12};
        App app = new App();
        app.compute(input, input);
        // must be 2
        assertArrayEquals(new int[]{1}, input);
    }
}
