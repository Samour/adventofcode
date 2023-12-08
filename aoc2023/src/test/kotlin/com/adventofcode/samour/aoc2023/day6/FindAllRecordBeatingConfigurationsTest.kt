package com.adventofcode.samour.aoc2023.day6

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test
import java.math.BigInteger

class FindAllRecordBeatingConfigurationsTest {

    @Test
    fun `Should return 288 for sample dataset`() {
        val winningConfigurations = findAllRecordBeatingConfigurations("sample.txt")
        val winningConfigurationsProduct = winningConfigurations.reduce { acc, i -> acc * i }

        assertThat(winningConfigurations).containsExactly(
            "4".toBigInteger(),
            "8".toBigInteger(),
            "9".toBigInteger(),
        )
        assertThat(winningConfigurationsProduct).isEqualTo("288".toBigInteger())
    }

    @Test
    fun `Should return 3316275 for problem dataset`() {
        val result = findAllRecordBeatingConfigurations("data.txt").reduce { acc, i -> acc * i }

        assertThat(result).isEqualTo("3316275".toBigInteger())
    }

    @Test
    fun `Should return 71503 for sample dataset with single race`() {
        val result = findRecordBeatingConfigurationForSingleRace("sample.txt")

        assertThat(result).isEqualTo("71503".toBigInteger())
    }

    @Test
    fun `Should return 27102791 for sample dataset with single race`() {
        val result = findRecordBeatingConfigurationForSingleRace("data.txt")

        assertThat(result).isEqualTo("27102791".toBigInteger())
    }
}
