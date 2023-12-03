package com.adventofcode.samour.aoc2023.day3

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun sumAdjacentNumbers(fname: String) = readResource("day3/$fname").use { file ->
    file.parseEngineSchematic()
        .findNumbersAdjacentToAnySymbol()
        .sum()
}

fun sumGearRatios(fname: String) = readResource("day3/$fname").use { file ->
    val schematic = file.parseEngineSchematic()
    val starPositions = schematic.symbols.entries
        .filter { (_, c) -> c == '*' }
        .map { (pos, _) -> pos }
    starPositions.map { (x, y) -> schematic.findNumbersAdjacentToPosition(x, y) }
        .filter { it.size == 2 }
        .sumOf { (first, second) -> first * second }
}
