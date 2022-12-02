package com.adventofcode.samour.aoc2022.day2.parser

import com.adventofcode.samour.aoc2022.day2.RPSShape.*

typealias StrategyParser = (tokenisedRound: TokenisedRound) -> RoundStrategy

fun parseStrategyByShape(tokenisedRound: TokenisedRound): RoundStrategy = RoundStrategy(
    opponentShape = when (tokenisedRound.opponentShapeToken) {
        "A" -> ROCK
        "B" -> PAPER
        "C" -> SCISSORS
        else -> throw IllegalArgumentException("Do not recognize token '${tokenisedRound.opponentShapeToken}'")
    },
    playerShape = when (tokenisedRound.playerShapeToken) {
        "X" -> ROCK
        "Y" -> PAPER
        "Z" -> SCISSORS
        else -> throw IllegalArgumentException("Do not recognize token '${tokenisedRound.playerShapeToken}'")
    },
)
