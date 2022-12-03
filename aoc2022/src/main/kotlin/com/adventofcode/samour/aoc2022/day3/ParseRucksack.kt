package com.adventofcode.samour.aoc2022.day3

import com.adventofcode.samour.aoc2022.day3.CompartmentType.FIRST
import com.adventofcode.samour.aoc2022.day3.CompartmentType.SECOND

fun parseRucksack(line: String): Rucksack = line.trim().let {
    Rucksack(
        mapOf(
            FIRST to it.subSequence(0, it.length / 2).toList(),
            SECOND to it.subSequence(it.length / 2, it.length).toList(),
        )
    )
}
