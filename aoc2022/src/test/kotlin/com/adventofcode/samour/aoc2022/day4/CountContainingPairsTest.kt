package com.adventofcode.samour.aoc2022.day4

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class CountContainingPairsTest {

    @Test
    fun `should count containing pairs in sample file`() {
        val result = countContainingPairs("sample.txt")

        assertThat(result).isEqualTo(2)
    }

    @Test
    fun `should count containing pairs in data file`() {
        val result = countContainingPairs("data.txt")

        assertThat(result).isEqualTo(532)
    }
}
