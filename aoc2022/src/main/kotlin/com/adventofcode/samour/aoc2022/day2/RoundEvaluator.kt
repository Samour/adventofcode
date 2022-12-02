package com.adventofcode.samour.aoc2022.day2

import com.adventofcode.samour.aoc2022.day2.RPSShape.*

private val defeats = mapOf(
    PAPER to ROCK,
    ROCK to SCISSORS,
    SCISSORS to PAPER,
)

fun evaluateRound(playerShape: RPSShape, opponentShape: RPSShape): RoundResult =
    if (playerShape == opponentShape) {
        RoundResult.Draw(playerShape)
    } else if (defeats[playerShape] == opponentShape) {
        RoundResult.Win(playerShape)
    } else {
        RoundResult.Loss(playerShape)
    }
