package com.adventofcode.samour.aoc2023.day8

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class FindRouteTest {

    @Test
    fun `Should return 2 steps for sample dataset`() {
        val result = findRoute("sample.txt", false)

        assertThat(result).isEqualTo(2)
    }

    @Test
    fun `Should return 6 steps for second sample dataset`() {
        val result = findRoute("sample2.txt", false)

        assertThat(result).isEqualTo(6)
    }

    @Test
    fun `Should return 12643 steps for problem dataset`() {
        val result = findRoute("data.txt", false)

        assertThat(result).isEqualTo(12643)
    }

    @Test
    fun `Should return 6 steps for simultaneously navigating third sample dataset`() {
        val result = findRoute("sample3.txt", true)

        assertThat(result).isEqualTo(6)
    }

    @Test
    fun `Should return X steps for simultaneously navigating problem dataset`() {
        val result = findRoute("data.txt", true)

        assertThat(result).isEqualTo(0)
    }
}
