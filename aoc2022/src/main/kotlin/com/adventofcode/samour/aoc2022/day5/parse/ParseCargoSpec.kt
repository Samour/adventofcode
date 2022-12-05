package com.adventofcode.samour.aoc2022.day5.parse

import com.adventofcode.samour.aoc2022.day5.CargoStructure
import com.adventofcode.samour.aoc2022.day5.MoveInstruction
import java.io.BufferedReader

data class CargoSpec(
    val initialPosition: CargoStructure,
    val movements: List<MoveInstruction>,
)

fun parseCargoSpec(reader: BufferedReader): CargoSpec {
    val structureLines = mutableListOf<String>()
    val movementLines = mutableListOf<String>()
    var readingStructure = true
    reader.lineSequence().forEach {
        if (it.trim().isEmpty()) {
            readingStructure = false
        } else if (readingStructure) {
            structureLines.add(it)
        } else {
            movementLines.add(it)
        }
    }

    return CargoSpec(
        initialPosition = parseCargoStructure(
            structureLines.subList(0, structureLines.size - 1),
            structureLines.last(),
        ),
        movements = movementLines.map { parseMoveInstruction(it) },
    )
}
