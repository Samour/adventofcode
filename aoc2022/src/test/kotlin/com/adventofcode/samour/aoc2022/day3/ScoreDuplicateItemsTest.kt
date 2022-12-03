package com.adventofcode.samour.aoc2022.day3

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class ScoreDuplicateItemsTest {

    @Test
    fun `should correctly score duplicates in sample dataset`() {
        val result = scoreDuplicateItems("sample.txt")

        assertThat(result).isEqualTo(157)
    }

    @Test
    fun `should correctly score duplicates in problem dataset`() {
        val result = scoreDuplicateItems("data.txt")

        assertThat(result).isEqualTo(7997)
    }
}
