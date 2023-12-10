package com.adventofcode.samour.aoc2023.day10

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class PipeSpecTest {

    @Test
    fun `Should filter out unexpected sections`() {
        val spec = readResource("day10/sample.txt").use { it.parsePipeSpec() }
        val withoutLoops = spec.withoutLoops()

        assertThat(withoutLoops).isNull()
    }
}
