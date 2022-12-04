package com.adventofcode.samour.aoc2022.day4

typealias OverlappingPairCheck = (pair: CleaningPair) -> Boolean

fun pairFullyContained(pair: CleaningPair) =
    pair.first.contains(pair.second) || pair.second.contains(pair.first)

private fun CampSection.contains(other: CampSection) =
    rangeStart <= other.rangeStart && rangeEnd >= other.rangeEnd
