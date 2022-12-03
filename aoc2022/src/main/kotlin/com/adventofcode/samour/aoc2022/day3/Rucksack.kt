package com.adventofcode.samour.aoc2022.day3

enum class CompartmentType {
    FIRST,
    SECOND,
}

data class Rucksack(
    val compartments: Map<CompartmentType, List<Char>>,
)
