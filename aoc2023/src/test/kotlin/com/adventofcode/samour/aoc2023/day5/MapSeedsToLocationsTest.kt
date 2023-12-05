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
        val plantingDetails = loadMaps("sample.txt").let {
            if (useCollapsedMaps) {
                it.copy(
                    attributeMapChain = collapseMaps(it.attributeMapChain),
                )
            } else {
                it
            }
        }

        val result = mapSeedsToLocations(plantingDetails)
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
        val plantingDetails = loadMaps("data.txt").let {
            if (useCollapsedMaps) {
                it.copy(
                    attributeMapChain = collapseMaps(it.attributeMapChain),
                )
            } else {
                it
            }
        }

        val result = mapSeedsToLocations(plantingDetails).values.min()

        assertThat(result).isEqualTo("825516882".toBigInteger())
    }
}
