package com.adventofcode.samour.aoc2023.day5

import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.SoftAssertions.assertSoftly
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.ValueSource

class MapSeedsToLocationsTest {

    @ParameterizedTest
    @ValueSource(booleans = [false, true])
    fun `Should return correct mappings for seeds`(useCollapsedMaps: Boolean) {
        val result = mapSeedsToLocations("sample.txt", useCollapsedMaps)
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

    @ParameterizedTest
    @ValueSource(booleans = [false, true])
    fun `Should return 825516882 for problem dataset`(useCollapsedMaps: Boolean) {
        val result = mapSeedsToLocations("data.txt", useCollapsedMaps).values.min()

        assertThat(result).isEqualTo("825516882".toBigInteger())
    }

    @Test
    fun `Should return 46 for range of seeds in sample dataset`() {
        val result = findClosestLocationForSeedRanges("sample.txt")

        assertThat(result).isEqualTo(46)
    }

    @Test
    fun `Should return 136096660 for range of seeds in problem dataset`() {
        val result = findClosestLocationForSeedRanges("data.txt")

        assertThat(result).isEqualTo("136096660".toBigInteger())
    }
}
