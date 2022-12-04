package com.adventofcode.samour.aoc2022.day4

import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun countOverlappingPairs(fname: String, overlappingPairCheck: OverlappingPairCheck) =
    readResource("day4/$fname")
        .lineSequence()
        .map { parseCleaningPairs(it) }
        .count { overlappingPairCheck(it) }
