package com.adventofcode.samour.aoc2023.day7

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class CalculateGameScoreTest {

    @Test
    fun `Should return 6440 for sample dataset`() {
        val result = calculateGameScore("sample.txt")

        assertThat(result).isEqualTo(6440)
    }

    @Test
    fun `Should return 253954294 for problem dataset`() {
        val result = calculateGameScore("data.txt")

        assertThat(result).isEqualTo(253954294)
    }
}
