package com.adventofcode.samour.aoc2022.day4

fun parseCleaningPairs(spec: String): CleaningPair {
    val sectionSpec = spec.split(",")
    return CleaningPair(
        first = parseCampSection(sectionSpec[0]),
        second = parseCampSection(sectionSpec[1]),
    )
}

private fun parseCampSection(spec: String) = spec.split("-").let {
    CampSection(
        rangeStart = it[0].toInt(),
        rangeEnd = it[1].toInt(),
    )
}
