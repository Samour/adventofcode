package com.adventofcode.samour.aoc2023.day9

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class NextValueInOasisSequenceTest {

    @Test
    fun `Should return correct next value in sequence for example given`() {
        val result = nextValueInOasisSequence(
            listOf(10, 13, 16, 21, 30, 45)
        )

        assertThat(result).isEqualTo(68)
    }
}
