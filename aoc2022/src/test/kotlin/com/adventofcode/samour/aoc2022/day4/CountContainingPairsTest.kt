package com.adventofcode.samour.aoc2022.day4

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class CountContainingPairsTest {

    @Test
    fun `should count containing pairs in sample file`() {
        val result = countOverlappingPairs("sample.txt", ::pairFullyContained)

        assertThat(result).isEqualTo(2)
    }

    @Test
    fun `should count containing pairs in data file`() {
        val result = countOverlappingPairs("data.txt", ::pairFullyContained)

        assertThat(result).isEqualTo(532)
    }

    @Test
    fun `should count overlapping pairs in sample file`() {
        val result = countOverlappingPairs("sample.txt", ::pairOverlaps)

        assertThat(result).isEqualTo(4)
    }

    @Test
    fun `should count overlapping pairs in data file`() {
        val result = countOverlappingPairs("data.txt", ::pairOverlaps)

        assertThat(result).isEqualTo(-1)
    }
}
