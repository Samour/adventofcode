package com.adventofcode.samour.aoc2023.day10

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun findDistanceFromStart(fname: String) = readResource("day10/$fname").use { file ->
    traceAnimalLoop(file.parsePipeSpec().withoutLoops()).stepsToFurthestPoint
}
