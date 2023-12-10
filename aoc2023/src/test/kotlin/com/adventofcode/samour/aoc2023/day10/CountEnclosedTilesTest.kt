package com.adventofcode.samour.aoc2023.day10

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class CountEnclosedTilesTest {

    @Test
    fun `Should return 4 for third sample dataset`() {
        val result = countEnclosedTiles("sample3.txt")

        assertThat(result).isEqualTo(4)
    }

    @Test
    fun `Should return 4 for fourth sample dataset`() {
        val result = countEnclosedTiles("sample4.txt")

        assertThat(result).isEqualTo(4)
    }

    @Test
    fun `Should return 8 for fifth sample dataset`() {
        val result = countEnclosedTiles("sample5.txt")

        assertThat(result).isEqualTo(8)
    }

    @Test
    fun `Should return 10 for sixth sample dataset`() {
        val result = countEnclosedTiles("sample6.txt")

        assertThat(result).isEqualTo(10)
    }

    @Test
    fun `Should return 429 for problem dataset`() {
        val result = countEnclosedTiles("data.txt")

        assertThat(result).isEqualTo(429)
    }
}
