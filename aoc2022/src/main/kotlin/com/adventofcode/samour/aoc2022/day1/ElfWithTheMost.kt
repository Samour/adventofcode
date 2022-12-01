package com.adventofcode.samour.aoc2022.day1

fun selectElfWithTheMost(fname: String): ElfCalorieSupply =
    selectElvesWithTheMost(fname, 1)[0]

fun selectElvesWithTheMost(fname: String, count: Int): List<ElfCalorieSupply> =
    loadCalories(fname).sortedByDescending { it.calories }
        .subList(0, count)
