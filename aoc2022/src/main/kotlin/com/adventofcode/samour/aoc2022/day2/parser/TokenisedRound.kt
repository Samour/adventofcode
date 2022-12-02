package com.adventofcode.samour.aoc2022.day2.parser

data class TokenisedRound(
    val opponentShapeToken: String,
    val playerShapeToken: String,
)

fun tokeniseRound(line: String) = line.trim().split(" ").let {
    TokenisedRound(
        opponentShapeToken = it[0],
        playerShapeToken = it[1],
    )
}
