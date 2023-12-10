package com.adventofcode.samour.aoc2023.day10

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class FindFurthestFromStartTest {

    @Test
    fun `Should return 4 for sample dataset`() {
        val result = findDistanceFromStart("sample.txt")

        assertThat(result).isEqualTo(4)
    }

    @Test
    fun `Should return 8 for second sample dataset`() {
        val result = findDistanceFromStart("sample2.txt")

        assertThat(result).isEqualTo(8)
    }

    @Test
    fun `Should return 6714 for problem dataset`() {
        val result = findDistanceFromStart("data.txt")

        assertThat(result).isEqualTo(6714)
    }
}
