package com.adventofcode.samour.aoc2022.day2

import com.adventofcode.samour.aoc2022.day2.parser.StrategyParser
import com.adventofcode.samour.aoc2022.day2.parser.tokeniseRound
import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun computeRPSScore(fname: String, strategyParser: StrategyParser) = readResource("day2/$fname")
    .lineSequence()
    .map { tokeniseRound(it) }
    .map { strategyParser(it) }
    .map { (opponentShape, playerShape) ->
        evaluateRound(
            playerShape = playerShape,
            opponentShape = opponentShape,
        )
    }.map { it.totalScore() }
    .sum()
