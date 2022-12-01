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
}
