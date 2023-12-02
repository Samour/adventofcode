package com.adventofcode.samour.aoc2023.day2

import com.adventofcode.samour.aoc2023.day2.CubeColour.GREEN
import com.adventofcode.samour.aoc2023.day2.CubeColour.RED
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

class AggregateViableGamesTest {

    @Test
    fun `Should return 8 for sample dataset and cubeset`() {
        val cubeset = CubeCounts(
            counts = mapOf(
                RED to 12,
                GREEN to 13,
                CubeColour.BLUE to 14,
            ),
        )
        val result = aggregateViableGames("sample.txt", cubeset)

        assertThat(result).isEqualTo(
            ViableGameIds(
                listOf(1, 2, 5),
            ),
        )
    }

    @Test
    fun `Should return 2061 for problem dataset and cubeset`() {
        val cubeset = CubeCounts(
            counts = mapOf(
                RED to 12,
                GREEN to 13,
                CubeColour.BLUE to 14,
            ),
        )
        val result = aggregateViableGames("data.txt", cubeset)

        assertThat(result.gameIdSum).isEqualTo(2061)
    }

    @Test
    fun `Power of min sample cubesets should be 2286`() {
        val powerSum = minViableCubeset("sample.txt")

        assertThat(powerSum).isEqualTo(2286)
    }

    @Test
    fun `Power of min data cubesets should be 72596`() {
        val powerSum = minViableCubeset("data.txt")

        assertThat(powerSum).isEqualTo(72596)
    }
}
