package com.adventofcode.samour.aoc2023.day4

import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.SoftAssertions.assertSoftly
import org.junit.jupiter.api.Test

class CountWinningCardsTest {

    @Test
    fun `Should return 13 points for sample dataset`() {
        val cards = countWinningCards("sample.txt")

        val cardScores = cards.map { it.score }
        assertSoftly { s ->
            s.assertThat(cardScores).containsExactly(
                8,
                2,
                2,
                1,
                0,
                0,
            )
            s.assertThat(cardScores.sum()).isEqualTo(13)
        }
    }

    @Test
    fun `Should return 17782 points for problem dataset`() {
        val result = countWinningCards("data.txt")
            .sumOf { it.score }

        assertThat(result).isEqualTo(17782)
    }

    @Test
    fun `Should return 30 cards for sample dataset`() {
        val copies = countCascadedCopies("sample.txt")

        val totalCopies = copies.values.sum()
        assertSoftly { s ->
            s.assertThat(copies).isEqualTo(
                mapOf(
                    1 to 1,
                    2 to 2,
                    3 to 4,
                    4 to 8,
                    5 to 14,
                    6 to 1,
                ),
            )
            s.assertThat(totalCopies).isEqualTo(30)
        }
    }

    @Test
    fun `Should return 8477787 cards for problem dataset`() {
        val result = countCascadedCopies("data.txt").values.sum()

        assertThat(result).isEqualTo(8477787)
    }
}
