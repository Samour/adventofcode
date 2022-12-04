package com.adventofcode.samour.aoc2022.day4

import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun countContainingPairs(fname: String) = readResource("day4/$fname")
    .lineSequence()
    .map { parseCleaningPairs(it) }
    .count { pairFullyContained(it) }
