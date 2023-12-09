package com.adventofcode.samour.aoc2023.day9

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class SumNextValuesInSequencesTest {

    @Test
    fun `Should return 114 for sample dataset`() {
        val result = sumNextValuesInSequences("sample.txt")

        assertThat(result).isEqualTo(114)
    }

    @Test
    fun `Should return 1916822650 for problem dataset`() {
        val result = sumNextValuesInSequences("data.txt")

        assertThat(result).isEqualTo(1916822650)
    }
}
