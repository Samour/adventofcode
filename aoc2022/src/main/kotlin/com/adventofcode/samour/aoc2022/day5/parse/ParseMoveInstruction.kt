package com.adventofcode.samour.aoc2022.day5.parse

import com.adventofcode.samour.aoc2022.day5.MoveInstruction

private val instructionPattern = Regex("move (\\d+) from (\\d+) to (\\d+)")

fun parseMoveInstruction(line: String): MoveInstruction =
    instructionPattern.matchEntire(line)?.let {
        MoveInstruction(
            // Application data structures are 0-indexed
            source = it.groupValues[2].toInt() - 1,
            destination = it.groupValues[3].toInt() - 1,
            quantity = it.groupValues[1].toInt(),
        )
    } ?: throw IllegalArgumentException("Could not interpret move instruction '$line'")
