package com.adventofcode.samour.aoc2022.day2.parser

import com.adventofcode.samour.aoc2022.day2.RPSShape
import com.adventofcode.samour.aoc2022.day2.RPSShape.*

typealias StrategyParser = (tokenisedRound: TokenisedRound) -> RoundStrategy

fun parseStrategyByShape(tokenisedRound: TokenisedRound): RoundStrategy = RoundStrategy(
    opponentShape = parseOpponentShape(tokenisedRound.opponentShapeToken),
    playerShape = when (tokenisedRound.playerShapeToken) {
        "X" -> ROCK
        "Y" -> PAPER
        "Z" -> SCISSORS
        else -> throw IllegalArgumentException("Do not recognize token '${tokenisedRound.playerShapeToken}'")
    },
)

fun parseStrategyByOutcome(tokenisedRound: TokenisedRound): RoundStrategy =
    parseOpponentShape(tokenisedRound.opponentShapeToken).let {
        RoundStrategy(
            opponentShape = it,
            playerShape = when (tokenisedRound.playerShapeToken) {
                "X" -> RPSShape.defeats[it]!!
                "Y" -> it
                "Z" -> RPSShape.defeatedBy[it]!!
                else -> throw IllegalArgumentException("Do not recognize token '${tokenisedRound.playerShapeToken}'")
            }
        )
    }

private fun parseOpponentShape(token: String) = when (token) {
    "A" -> ROCK
    "B" -> PAPER
    "C" -> SCISSORS
    else -> throw IllegalArgumentException("Do not recognize token '$token'")
}
