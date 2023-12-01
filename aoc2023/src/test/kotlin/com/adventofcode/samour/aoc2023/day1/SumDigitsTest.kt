package com.adventofcode.samour.aoc2023.day1

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class SumDigitsTest {

    @Test
    fun `Should return 142 for sample data using digit parsing`() {
        val result = sumDigits("sample.txt", ::filterOnlyDigits)

        assertThat(result).isEqualTo(142)
    }

    @Test
    fun `Should return 55108 for problem data using digit parsing`() {
        val result = sumDigits("data.txt", ::filterOnlyDigits)

        assertThat(result).isEqualTo(55108)
    }

    @Test
    fun `Should return 281 for sample data using digit and word parsing`() {
        val result = sumDigits("sample2.txt", ::filterDigitsAndWords)

        assertThat(result).isEqualTo(281)
    }

    @Test
    fun `Should return 56324 for problem data using digit and word parsing`() {
        val result = sumDigits("data.txt", ::filterDigitsAndWords)

        assertThat(result).isEqualTo(56324)
    }
}
