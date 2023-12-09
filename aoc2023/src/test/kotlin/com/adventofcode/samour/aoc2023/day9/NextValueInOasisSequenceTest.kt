package com.adventofcode.samour.aoc2023.day9

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class NextValueInOasisSequenceTest {

    @Test
    fun `Should return 68 for example given`() {
        val result = nextValueInOasisSequence(
            listOf(10, 13, 16, 21, 30, 45),
            false,
        )

        assertThat(result).isEqualTo(68)
    }

    @Test
    fun `Should return 5 when extrapolating backwards for example given`() {
        val result = nextValueInOasisSequence(
            listOf(10, 13, 16, 21, 30, 45),
            true,
        )

        assertThat(result).isEqualTo(5)
    }
}
