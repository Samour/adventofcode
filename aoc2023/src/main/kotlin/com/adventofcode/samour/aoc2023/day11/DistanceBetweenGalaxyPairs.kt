package com.adventofcode.samour.aoc2023.day11

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource
import kotlin.math.abs

fun findDistanceBetweenGalaxyPairs(fname: String) = readResource("day11/$fname").use { file ->
    file.parseGalaxyMap()
        .expandEmptySpaces()
        .getGalaxyCoordinatePairs()
        .sumOf { (first, second) -> orthoDistanceBetween(first, second) }
}

private fun GalaxyMap.getGalaxyCoordinatePairs(): Set<Pair<Pair<Int, Int>, Pair<Int, Int>>> {
    val galaxyCoords = rows.flatMapIndexed { y, row ->
        row.mapIndexedNotNull { x, galaxyState ->
            (x to y).takeIf { galaxyState == GalaxyState.PRESENT }
        }
    }

    return galaxyCoords.flatMapIndexed { i, coord ->
        (0 until i).map { j ->
            coord to galaxyCoords[j]
        }
    }.toSet()
}

private fun orthoDistanceBetween(first: Pair<Int, Int>, second: Pair<Int, Int>): Int {
    return abs(first.first - second.first) + abs(first.second - second.second)
}
