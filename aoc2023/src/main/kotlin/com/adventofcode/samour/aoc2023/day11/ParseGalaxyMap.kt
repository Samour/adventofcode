package com.adventofcode.samour.aoc2023.day11

import java.io.BufferedReader
import java.math.BigInteger

fun BufferedReader.parseGalaxyMap(expansionFactor: BigInteger): GalaxyMap = GalaxyMap(
    expansionFactor = expansionFactor,
    rows = readLines().map { line ->
        line.toCharArray().map {
            when (it) {
                '.' -> GalaxyState.NONE
                '#' -> GalaxyState.PRESENT
                else -> throw IllegalArgumentException("Unrecognized character: $it")
            }
        }
    },
)

fun GalaxyMap.render(): String = rows.joinToString("\n") { row ->
    row.joinToString("") {
        when (it) {
            GalaxyState.NONE -> "."
            GalaxyState.PRESENT -> "#"
        }
    }
}
