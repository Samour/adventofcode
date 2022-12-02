package com.adventofcode.samour.aoc2022.day2

fun evaluateRound(playerShape: RPSShape, opponentShape: RPSShape): RoundResult =
    if (playerShape == opponentShape) {
        RoundResult.Draw(playerShape)
    } else if (RPSShape.defeats[playerShape] == opponentShape) {
        RoundResult.Win(playerShape)
    } else {
        RoundResult.Loss(playerShape)
    }
