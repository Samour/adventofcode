package com.adventofcode.samour.aoc2023.day9

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun sumNextValuesInSequences(fname: String): Int = readResource("day9/$fname").use { file ->
    file.parseOasisSequences()
        .sumOf { nextValueInOasisSequence(it) }
}
