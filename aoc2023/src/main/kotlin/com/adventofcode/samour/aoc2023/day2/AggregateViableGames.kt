package com.adventofcode.samour.aoc2023.day2

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun aggregateViableGames(fname: String, cubeset: CubeCounts) = readResource("day2/$fname").use { file ->
    ViableGameIds(
        file.parseCubeGames()
            .filter(gameViableForCubeset(cubeset))
            .map { it.gameId },
    )
}

data class ViableGameIds(val gameIds: List<Int>) {
    val gameIdSum = gameIds.sum()
}
