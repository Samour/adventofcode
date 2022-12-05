package com.adventofcode.samour.aoc2022.day5

import com.adventofcode.samour.aoc2022.day5.parse.parseCargoSpec
import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun computeMovements(fname: String): String {
    val (initialPosition, movements) = parseCargoSpec(readResource("day5/$fname"))
    return movements.fold(initialPosition) { structure, moveInstruction ->
        structure.applyMoveInstruction(moveInstruction)
    }.let { composeTopContainers(it) }
}

private fun composeTopContainers(structure: CargoStructure): String =
    structure.stacks.joinToString("") {
        "${it.firstOrNull() ?: " "}"
    }
