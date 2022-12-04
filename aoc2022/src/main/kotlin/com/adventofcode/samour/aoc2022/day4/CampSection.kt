package com.adventofcode.samour.aoc2022.day4

data class CampSection(
    val rangeStart: Int,
    val rangeEnd: Int,
)

data class CleaningPair(
    val first: CampSection,
    val second: CampSection,
)
