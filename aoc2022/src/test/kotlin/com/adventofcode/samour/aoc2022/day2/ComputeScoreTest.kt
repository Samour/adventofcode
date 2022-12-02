package com.adventofcode.samour.aoc2022.day2

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class ComputeScoreTest {

    @Test
    fun `should compute total score for sample strategy`() {
        val score = computeRPSScore("sample.txt")

        assertThat(score).isEqualTo(15)
    }

    @Test
    fun `should compute total score for problem strategy`() {
        val score = computeRPSScore("data.txt")

        assertThat(score).isEqualTo(13268)
    }
}
