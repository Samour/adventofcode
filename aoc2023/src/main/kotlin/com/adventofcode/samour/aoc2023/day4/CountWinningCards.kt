package com.adventofcode.samour.aoc2023.day4

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun countWinningCards(fname: String) = readResource("day4/$fname").use { file ->
    file.parseScratchCards()
}
