package com.adventofcode.samour.aoc2022.day1

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class ElfWithTheMostKtTest {

    @Test
    fun `select elf from sample file`() {
        val elf = selectElfWithTheMost("sample.txt")

        assertThat(elf).isEqualTo(ElfCalorieSupply(24_000))
    }

    @Test
    fun `select elf from data file`() {
        val elf = selectElfWithTheMost("data.txt")

        assertThat(elf).isEqualTo(ElfCalorieSupply(69_912))
    }

    @Test
    fun `select top 3 elves from sample file`() {
        val totalCalories = selectElvesWithTheMost("sample.txt", 3)
            .sumOf { it.calories }

        assertThat(totalCalories).isEqualTo(45_000)
    }

    @Test
    fun `select top 3 elves from data file`() {
        val totalCalories = selectElvesWithTheMost("data.txt", 3)
            .sumOf { it.calories }

        assertThat(totalCalories).isEqualTo(208_180)
    }
}
