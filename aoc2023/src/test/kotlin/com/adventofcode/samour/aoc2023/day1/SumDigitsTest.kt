package com.adventofcode.samour.aoc2023.day1

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class SumDigitsTest {

    @Test
    fun `Should return 142 for sample data`() {
        val result = sumDigits("sample.txt")

        assertThat(result).isEqualTo(142)
    }

    @Test
    fun `Should return 55108 for problem data`() {
        val result = sumDigits("data.txt")

        assertThat(result).isEqualTo(55108)
    }
}
