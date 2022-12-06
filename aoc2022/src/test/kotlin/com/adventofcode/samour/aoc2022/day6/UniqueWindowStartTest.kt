package com.adventofcode.samour.aoc2022.day6

import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class UniqueWindowStartTest {

    @Test
    fun `should find packet start in first sample data`() {
        val result = findUniqueWindowStart("bvwbjplbgvbhsrlpgdmjqwftvncz", 4)

        assertThat(result).isEqualTo(5)
    }

    @Test
    fun `should find packet start in second sample data`() {
        val result = findUniqueWindowStart("nppdvjthqldpwncqszvftbrmjlhg", 4)

        assertThat(result).isEqualTo(6)
    }

    @Test
    fun `should find packet start in third sample data`() {
        val result = findUniqueWindowStart("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)

        assertThat(result).isEqualTo(10)
    }

    @Test
    fun `should find packet start in fourth sample data`() {
        val result = findUniqueWindowStart("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)

        assertThat(result).isEqualTo(11)
    }

    @Test
    fun `should find packet start in problem data`() {
        val result = findUniqueWindowStart(readResource("day6/data.txt").readText(), 4)

        assertThat(result).isEqualTo(1542)
    }

    @Test
    fun `should find message start in first sample data`() {
        val result = findUniqueWindowStart("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)

        assertThat(result).isEqualTo(19)
    }

    @Test
    fun `should find message start in second sample data`() {
        val result = findUniqueWindowStart("bvwbjplbgvbhsrlpgdmjqwftvncz", 14)

        assertThat(result).isEqualTo(23)
    }

    @Test
    fun `should find message start in third sample data`() {
        val result = findUniqueWindowStart("nppdvjthqldpwncqszvftbrmjlhg", 14)

        assertThat(result).isEqualTo(23)
    }

    @Test
    fun `should find message start in fourth sample data`() {
        val result = findUniqueWindowStart("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)

        assertThat(result).isEqualTo(29)
    }

    @Test
    fun `should find message start in fifth sample data`() {
        val result = findUniqueWindowStart("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)

        assertThat(result).isEqualTo(26)
    }

    @Test
    fun `should find message start in problem data`() {
        val result = findUniqueWindowStart(readResource("day6/data.txt").readText(), 14)

        assertThat(result).isEqualTo(3153)
    }
}
