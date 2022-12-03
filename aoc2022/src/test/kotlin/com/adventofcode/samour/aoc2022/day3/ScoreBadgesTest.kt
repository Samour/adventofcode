package com.adventofcode.samour.aoc2022.day3

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class ScoreBadgesTest {

    @Test
    fun `should find badges in sample dataset`() {
        val result = scoreBadges("sample.txt")

        assertThat(result).isEqualTo(70)
    }

    @Test
    fun `should find badges in problem dataset`() {
        val result = scoreBadges("data.txt")

        assertThat(result).isEqualTo(2545)
    }
}
