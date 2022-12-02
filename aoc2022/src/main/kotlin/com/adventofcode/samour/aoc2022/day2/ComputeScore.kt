package com.adventofcode.samour.aoc2022.day2

import com.adventofcode.samour.aoc2022.day2.parser.parseStrategy
import com.adventofcode.samour.aoc2022.day2.parser.tokeniseRound
import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun computeRPSScore(fname: String) = readResource("day2/$fname")
    .lineSequence()
    .map { tokeniseRound(it) }
    .map { parseStrategy(it) }
    .map { (opponentShape, playerShape) ->
        evaluateRound(
            playerShape = playerShape,
            opponentShape = opponentShape,
        )
    }.map { it.totalScore() }
    .sum()
