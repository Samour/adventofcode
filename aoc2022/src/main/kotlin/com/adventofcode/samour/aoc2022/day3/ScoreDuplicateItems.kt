package com.adventofcode.samour.aoc2022.day3

import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun scoreDuplicateItems(fname: String) = readResource("day3/$fname").lineSequence()
    .map { parseRucksack(it) }
    .map { selectDuplicateItem(it) }
    .map { scoreItem(it) }
    .sum()
