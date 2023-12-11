package com.adventofcode.samour.aoc2023.day11

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class GalaxyMapTest {

    @Test
    fun `Should correctly expand empty spaces`() {
        val result = readResource("day11/sample.txt").use { it.parseGalaxyMap() }
            .expandEmptySpaces()
            .render()

        val expected = readResource("day11/sample_expanded.txt").use { it.readText() }
        assertThat(result).isEqualTo(expected)
    }
}
