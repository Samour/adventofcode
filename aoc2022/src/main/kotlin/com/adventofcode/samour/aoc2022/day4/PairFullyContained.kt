package com.adventofcode.samour.aoc2022.day4

fun pairFullyContained(pair: CleaningPair) =
    pair.first.contains(pair.second) || pair.second.contains(pair.first)

private fun CampSection.contains(other: CampSection) =
    rangeStart <= other.rangeStart && rangeEnd >= other.rangeEnd
