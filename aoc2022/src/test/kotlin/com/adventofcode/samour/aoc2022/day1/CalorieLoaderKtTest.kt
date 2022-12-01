package com.adventofcode.samour.aoc2022.day1

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class CalorieLoaderKtTest {

    @Test
    fun `should load Elf calories from file`() {
        val supplies = loadCalories("sample.txt")

        assertThat(supplies).containsExactly(
            ElfCalorieSupply(6_000),
            ElfCalorieSupply(4_000),
            ElfCalorieSupply(11_000),
            ElfCalorieSupply(24_000),
            ElfCalorieSupply(10_000),
        )
    }
}
