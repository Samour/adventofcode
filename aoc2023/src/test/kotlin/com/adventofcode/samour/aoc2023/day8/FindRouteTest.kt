package com.adventofcode.samour.aoc2023.day8

import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.SoftAssertions.assertSoftly
import org.junit.jupiter.api.Test

class FindRouteTest {

    @Test
    fun `Should return correct route for sample dataset`() {
        val result = findRoute("sample.txt")

        assertSoftly { s ->
            s.assertThat(result).containsExactly(
                "AAA",
                "CCC",
                "ZZZ",
            )
            s.assertThat(result.size - 1).isEqualTo(2)
        }
    }

    @Test
    fun `Should return 6 steps for second sample dataset`() {
        val result = findRoute("sample2.txt").size - 1

        assertThat(result).isEqualTo(6)
    }

    @Test
    fun `Should return X steps for problem dataset`() {
        val result = findRoute("data.txt").size - 1

        assertThat(result).isEqualTo(0)
    }
}
