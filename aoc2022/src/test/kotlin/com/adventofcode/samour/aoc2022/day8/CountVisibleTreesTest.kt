package com.adventofcode.samour.aoc2022.day8

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class CountVisibleTreesTest {

    @Test
    fun `should count visible trees in sample file`() {
        val result = countVisibleTrees("sample.txt")

        assertThat(result).isEqualTo(21)
    }

    @Test
    fun `should count visible trees in problem file`() {
        val result = countVisibleTrees("data.txt")

        assertThat(result).isEqualTo(1798)
    }
}
