package com.adventofcode.samour.aoc2022.day2

import com.adventofcode.samour.aoc2022.day2.parser.parseStrategyByShape
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class ComputeScoreTest {

    @Test
    fun `should compute total score for sample strategy using shape parsing`() {
        val score = computeRPSScore("sample.txt", ::parseStrategyByShape)

        assertThat(score).isEqualTo(15)
    }

    @Test
    fun `should compute total score for problem strategy using shape parsing`() {
        val score = computeRPSScore("data.txt", ::parseStrategyByShape)

        assertThat(score).isEqualTo(13268)
    }
}
