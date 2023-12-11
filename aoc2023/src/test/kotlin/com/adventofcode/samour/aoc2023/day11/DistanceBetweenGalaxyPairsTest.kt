package com.adventofcode.samour.aoc2023.day11

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class DistanceBetweenGalaxyPairsTest {

    @Test
    fun `Should return 374 for sample dataset`() {
        val result = findDistanceBetweenGalaxyPairs("sample.txt")

        assertThat(result).isEqualTo(374)
    }

    @Test
    fun `Should return 9591768 for problem dataset`() {
        val result = findDistanceBetweenGalaxyPairs("data.txt")

        assertThat(result).isEqualTo(9591768)
    }
}
