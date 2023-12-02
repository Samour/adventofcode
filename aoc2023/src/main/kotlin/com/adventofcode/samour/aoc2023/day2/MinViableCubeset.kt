package com.adventofcode.samour.aoc2023.day2

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun minViableCubeset(fname: String) = readResource("day2/$fname").use { file ->
    file.parseCubeGames()
        .sumOf {
            cubePower(
                constructViableCubeset(it.displays),
            )
        }
}

private fun cubePower(counts: CubeCounts) = counts.counts.values
    .reduce { acc, i -> acc * i }
