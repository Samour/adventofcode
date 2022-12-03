package com.adventofcode.samour.aoc2022.day3

import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun scoreBadges(fname: String) = readResource("day3/$fname").lineSequence()
    .map { parseRucksack(it) }
    .fold(listOf<MutableList<Rucksack>>()) { groupings, rucksack ->
        if (groupings.isEmpty() || groupings.last().size == 3) {
            groupings + listOf(mutableListOf(rucksack))
        } else {
            groupings.last().add(rucksack)
            groupings
        }
    }.asSequence()
    .map { findBadge(it) }
    .map { scoreItem(it) }
    .sum()
