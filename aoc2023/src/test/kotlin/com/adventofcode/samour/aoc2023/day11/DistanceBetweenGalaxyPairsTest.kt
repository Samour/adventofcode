package com.adventofcode.samour.aoc2023.day11

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource
import java.math.BigInteger

class DistanceBetweenGalaxyPairsTest {

    @ParameterizedTest
    @CsvSource(
        value = [
            "2,374",
            "10,1030",
            "100,8410",
        ],
    )
    fun `Should return correct values for sample dataset`(expansionFactor: BigInteger, expected: BigInteger) {
        val result = findDistanceBetweenGalaxyPairs("sample.txt", expansionFactor)

        assertThat(result).isEqualTo(expected)
    }

    @ParameterizedTest
    @CsvSource(
        value = [
            "2,9591768",
            "1000000,746962097860",
        ],
    )
    fun `Should return 9591768 for problem dataset`(expansionFactor: BigInteger, expected: BigInteger) {
        val result = findDistanceBetweenGalaxyPairs("data.txt", expansionFactor)

        assertThat(result).isEqualTo(expected)
    }
}
