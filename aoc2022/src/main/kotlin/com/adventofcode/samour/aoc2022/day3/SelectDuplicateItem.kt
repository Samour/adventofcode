package com.adventofcode.samour.aoc2022.day3

import com.adventofcode.samour.aoc2022.day3.CompartmentType.FIRST
import com.adventofcode.samour.aoc2022.day3.CompartmentType.SECOND

fun selectDuplicateItem(rucksack: Rucksack): Char {
    val firstItems = rucksack.compartments[FIRST]!!.toSet()
    return rucksack.compartments[SECOND]!!.first { firstItems.contains(it) }
}
