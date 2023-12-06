package com.adventofcode.samour.aoc2023.day6

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class FindAllRecordBeatingConfigurationsTest {

    @Test
    fun `Should return 288 for sample dataset`() {
        val winningConfigurations = findAllRecordBeatingConfigurations("sample.txt")
        val winningConfigurationsProduct = winningConfigurations.reduce { acc, i -> acc * i }

        assertThat(winningConfigurations).containsExactly(4, 8, 9)
        assertThat(winningConfigurationsProduct).isEqualTo(288)
    }

    @Test
    fun `Should return 3316275 for problem dataset`() {
        val result = findAllRecordBeatingConfigurations("data.txt").reduce { acc, i -> acc * i }

        assertThat(result).isEqualTo(3316275)
    }
}
