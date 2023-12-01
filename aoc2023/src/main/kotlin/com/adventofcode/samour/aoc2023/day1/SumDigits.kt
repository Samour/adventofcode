package com.adventofcode.samour.aoc2023.day1

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun sumDigits(fname: String, digitParser: DigitParser): Int = readResource("day1/$fname").use { file ->
    file.readDigits(digitParser)
            .sumOf { it.toTwoDigitNum() }
}
