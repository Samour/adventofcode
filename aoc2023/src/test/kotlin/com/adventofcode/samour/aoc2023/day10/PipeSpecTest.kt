package com.adventofcode.samour.aoc2023.day10

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class PipeSpecTest {

    @Test
    fun `Should filter out unexpected sections in sample dataset`() {
        val result = readResource("day10/sample.txt").use { it.parsePipeSpec() }
            .withoutLoops()

        val expectedMap = readResource("day10/sample_simplified.txt").readText()
        assertThat(result.debugRender()).isEqualTo(expectedMap)
    }

    @Test
    fun `Should filter out unexpected sections in second sample dataset`() {
        val result = readResource("day10/sample2.txt").use { it.parsePipeSpec() }
            .withoutLoops()

        val expectedMap = readResource("day10/sample2_simplified.txt").readText()
        assertThat(result.debugRender()).isEqualTo(expectedMap)
    }
}
