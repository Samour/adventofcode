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
    fun `Should return X for problem data`() {
        val result = sumAdjacentNumbers("data.txt")

        assertThat(result).isEqualTo(-1) // 550064
    }
}
