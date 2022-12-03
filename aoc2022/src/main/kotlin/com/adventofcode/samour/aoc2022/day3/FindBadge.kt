package com.adventofcode.samour.aoc2022.day3

fun findBadge(rucksacks: List<Rucksack>): Char {
    val items = rucksacks.map {
        it.compartments.values.flatMap { c -> c.toSet() }.toSet()
    }
    val candidateBadges = items[0].toMutableSet()
    items.subList(1, items.size).forEach {
        candidateBadges.removeIf { i -> !it.contains(i) }
    }

    return candidateBadges.first()
}
