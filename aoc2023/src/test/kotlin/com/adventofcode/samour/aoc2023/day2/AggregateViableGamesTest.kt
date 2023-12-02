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
    fun `Should return X for problem dataset and cubeset`() {
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
}
