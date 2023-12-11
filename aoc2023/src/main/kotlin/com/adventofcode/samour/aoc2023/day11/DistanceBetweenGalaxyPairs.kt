package com.adventofcode.samour.aoc2023.day11

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource
import java.math.BigInteger

fun findDistanceBetweenGalaxyPairs(fname: String, expansionFactor: BigInteger) =
    readResource("day11/$fname").use { file ->
        file.parseGalaxyMap(expansionFactor)
            .getGalaxyCoordinatePairs()
            .sumOf { (first, second) -> orthoDistanceBetween(first, second) }
    }

private fun GalaxyMap.getGalaxyCoordinatePairs(): Set<Pair<Pair<BigInteger, BigInteger>, Pair<BigInteger, BigInteger>>> {
    return galaxyCoordinates.flatMapIndexed { i, coord ->
        (0 until i).map { j ->
            coord to galaxyCoordinates[j]
        }
    }.toSet()
}

private fun orthoDistanceBetween(first: Pair<BigInteger, BigInteger>, second: Pair<BigInteger, BigInteger>): BigInteger {
    return (first.first - second.first).abs() + (first.second - second.second).abs()
}
