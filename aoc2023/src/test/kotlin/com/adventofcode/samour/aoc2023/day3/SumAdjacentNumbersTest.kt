package com.adventofcode.samour.aoc2023.day3

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class SumAdjacentNumbersTest {

    @Test
    fun `Should return 4361 for sample data`() {
        val result = sumAdjacentNumbers("sample.txt")

        assertThat(result).isEqualTo(4361)
    }

    @Test
    fun `Should return 550064 for problem data`() {
        val result = sumAdjacentNumbers("data.txt")

        assertThat(result).isEqualTo(550064)
    }

    @Test
    fun `Should return 467835 for gear ratios of sample data`() {
        val result = sumGearRatios("sample.txt")

        assertThat(result).isEqualTo(467835)
    }

    @Test
    fun `Should return 85010461 for gear ratios of sample data`() {
        val result = sumGearRatios("data.txt")

        assertThat(result).isEqualTo(85010461)
    }
}
