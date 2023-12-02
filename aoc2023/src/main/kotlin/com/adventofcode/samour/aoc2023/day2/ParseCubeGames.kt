package com.adventofcode.samour.aoc2023.day2

import java.io.BufferedReader

fun BufferedReader.parseCubeGames(): List<CubeGame> = readLines().map { line ->
    val (gameInfo, cubeDisplays) = line.split(":")
    val gameNo = gameInfo.substring(5).toInt()

    CubeGame(
        gameId = gameNo,
        displays = cubeDisplays.split(";")
            .map(::parseCubeCounts),
    )
}

private fun parseCubeCounts(data: String): CubeCounts {
    return CubeCounts(
        counts = data.split(",")
            .associate(::parseColourCount),
    )
}

private fun parseColourCount(data: String): Pair<CubeColour, Int> {
    val (count, colour) = data.trim().split(" ")
    return CubeColour.valueOf(colour.uppercase()) to count.toInt()
}
