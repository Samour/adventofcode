package com.adventofcode.samour.aoc2023.day2

fun gameViableForCubeset(cubeset: CubeCounts): (CubeGame) -> Boolean = { game ->
    game.displays.all(displayViableForCubeset(cubeset))
}

private fun displayViableForCubeset(cubeset: CubeCounts): (CubeCounts) -> Boolean = { display ->
    display.counts.all { (colour, count) ->
        cubeset.counts[colour]?.let { it >= count } ?: false
    }
}
