package com.adventofcode.samour.aoc2022.day5

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class ComputeMovementsTest {

    @Test
    fun `should compute final position for sample data using single movements`() {
        val result = computeMovements("sample.txt", ::moveOneAtATime)

        assertThat(result).isEqualTo("CMZ")
    }

    @Test
    fun `should compute final position for problem data using single movements`() {
        val result = computeMovements("data.txt", ::moveOneAtATime)

        assertThat(result).isEqualTo("FJSRQCFTN")
    }

    @Test
    fun `should compute final position for sample data using multiple movements`() {
        val result = computeMovements("sample.txt", ::moveMultipleAtOnce)

        assertThat(result).isEqualTo("MCD")
    }

    @Test
    fun `should compute final position for problem data using multiple movements`() {
        val result = computeMovements("data.txt", ::moveMultipleAtOnce)

        assertThat(result).isEqualTo("CJVLJQPHS")
    }
}
