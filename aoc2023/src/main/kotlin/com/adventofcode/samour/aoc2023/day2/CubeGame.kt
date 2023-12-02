package com.adventofcode.samour.aoc2023.day2

data class CubeGame(
    val gameId: Int,
    val displays: List<CubeCounts>,
)

data class CubeCounts(
    val counts: Map<CubeColour, Int>,
)

enum class CubeColour {
    GREEN,
    RED,
    BLUE,
}
