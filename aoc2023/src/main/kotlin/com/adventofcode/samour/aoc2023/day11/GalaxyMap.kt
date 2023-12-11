package com.adventofcode.samour.aoc2023.day11

import java.math.BigInteger

data class GalaxyMap(
    val expansionFactor: BigInteger,
    val rows: List<List<GalaxyState>>,
) {

    val galaxyCoordinates: List<Pair<BigInteger, BigInteger>> by lazy {
        val baseCoords = rows.flatMapIndexed { y, row ->
            row.mapIndexedNotNull { x, galaxyState ->
                (x.toBigInteger() to y.toBigInteger()).takeIf {
                    galaxyState == GalaxyState.PRESENT
                }
            }
        }

        val xExpansions = (0 until rows.first().size).mapNotNull { x ->
            x.takeIf { rows.all { it[x] == GalaxyState.NONE } }
                ?.toBigInteger()
        }
        val yExpansions = rows.mapIndexedNotNull { y, row ->
            y.takeIf { row.all { it == GalaxyState.NONE } }
                ?.toBigInteger()
        }

        baseCoords.map { (x, y) ->
            val expandedX = x + xExpansions.count { it < x }
                .toBigInteger()
                .times(expansionFactor - BigInteger.ONE)
            val expandedY = y + yExpansions.count { it < y }
                .toBigInteger()
                .times(expansionFactor - BigInteger.ONE)

            expandedX to expandedY
        }
    }
}

enum class GalaxyState {
    PRESENT,
    NONE,
}
