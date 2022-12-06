package com.adventofcode.samour.aoc2022.day6

import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class FindPacketStartTest {

    @Test
    fun `should find packet start in first sample data`() {
        val result = findPacketStart("bvwbjplbgvbhsrlpgdmjqwftvncz")

        assertThat(result).isEqualTo(5)
    }

    @Test
    fun `should find packet start in second sample data`() {
        val result = findPacketStart("nppdvjthqldpwncqszvftbrmjlhg")

        assertThat(result).isEqualTo(6)
    }

    @Test
    fun `should find packet start in third sample data`() {
        val result = findPacketStart("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")

        assertThat(result).isEqualTo(10)
    }

    @Test
    fun `should find packet start in fourth sample data`() {
        val result = findPacketStart("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")

        assertThat(result).isEqualTo(11)
    }

    @Test
    fun `should find packet start in problem data`() {
        val result = findPacketStart(readResource("day6/data.txt").readText())

        assertThat(result).isEqualTo(1542)
    }
}
