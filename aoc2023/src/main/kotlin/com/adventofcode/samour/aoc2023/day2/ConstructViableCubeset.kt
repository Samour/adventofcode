package com.adventofcode.samour.aoc2023.day2

import kotlin.math.max

fun constructViableCubeset(displays: List<CubeCounts>): CubeCounts {
    val counts = mutableMapOf<CubeColour, Int>()
    displays.forEach { display ->
        display.counts.forEach { (colour, count) ->
            counts[colour] = counts[colour]?.let { max(it, count) } ?: count
        }
    }

    return CubeCounts(
        counts = counts,
    )
}
