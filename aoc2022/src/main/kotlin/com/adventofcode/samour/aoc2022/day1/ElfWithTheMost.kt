package com.adventofcode.samour.aoc2022.day1

fun selectElfWithTheMost(fname: String): ElfCalorieSupply =
    loadCalories(fname).maxBy { it.calories }
