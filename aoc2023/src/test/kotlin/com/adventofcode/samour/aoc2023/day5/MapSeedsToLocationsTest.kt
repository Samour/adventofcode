package com.adventofcode.samour.aoc2023.day5

import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.SoftAssertions.assertSoftly
import org.junit.jupiter.api.Test

class MapSeedsToLocationsTest {

    @Test
    fun `Should return correct mappings for seeds`() {
        val result = mapSeedsToLocations("sample.txt")
        val minLocation = result.values.min()

        assertSoftly { s ->
            s.assertThat(result).isEqualTo(
                mapOf(
                    79 to 82,
                    14 to 43,
                    55 to 86,
                    13 to 35,
                ).map { (k, v) -> k.toBigInteger() to v.toBigInteger() }.toMap(),
            )
            s.assertThat(minLocation).isEqualTo(35)
        }
    }

    @Test
    fun `Should return 825516882 for problem dataset`() {
        val result = mapSeedsToLocations("data.txt").values.min()

        assertThat(result).isEqualTo(825516882)
    }
}
