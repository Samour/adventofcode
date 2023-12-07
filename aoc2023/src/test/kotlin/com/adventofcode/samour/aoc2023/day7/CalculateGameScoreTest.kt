package com.adventofcode.samour.aoc2023.day7

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class CalculateGameScoreTest {

    @Test
    fun `Should return 6440 for sample dataset`() {
        val result = calculateGameScore("sample.txt", false)

        assertThat(result).isEqualTo("6440".toBigInteger())
    }

    @Test
    fun `Should return 253954294 for problem dataset`() {
        val result = calculateGameScore("data.txt", false)

        assertThat(result).isEqualTo("253954294".toBigInteger())
    }

    @Test
    fun `Should return 5905 for sample dataset using jokers`() {
        val result = calculateGameScore("sample.txt", true)

        assertThat(result).isEqualTo("5905".toBigInteger())
    }

    @Test
    fun `Should return X for problem dataset using jokers`() {
        val result = calculateGameScore("data.txt", true)

        assertThat(result).isEqualTo("254837398".toBigInteger())
    }
}
