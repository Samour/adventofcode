package com.adventofcode.samour.aoc2022.day1

import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun loadCalories(fname: String): List<ElfCalorieSupply> {
    val supplies = mutableListOf<ElfCalorieSupply>()
    var currentSupply = ElfCalorieSupply(0)
    readResource("day1/$fname").forEachLine {
        if (it.trim().isEmpty()) {
            if (currentSupply.calories > 0) {
                supplies.add(currentSupply)
            }
            currentSupply = ElfCalorieSupply(0)
        } else {
            currentSupply = ElfCalorieSupply(currentSupply.calories + it.toInt())
        }
    }

    if (currentSupply.calories > 0) {
        supplies.add(currentSupply)
    }

    return supplies
}
