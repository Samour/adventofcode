package com.adventofcode.samour.aoc2023.day1

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun sumDigits(fname: String): Int = readResource("day1/$fname").use { file ->
    file.readDigits()
            .map { it.toTwoDigitNum() }
            .sum()
}
