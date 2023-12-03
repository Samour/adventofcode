package com.adventofcode.samour.aoc2023.day3

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun sumAdjacentNumbers(fname: String) = readResource("day3/$fname").use { file ->
    file.parseEngineSchematic()
        .findNumbersAdjacentToSymbol()
        .sum()
}
