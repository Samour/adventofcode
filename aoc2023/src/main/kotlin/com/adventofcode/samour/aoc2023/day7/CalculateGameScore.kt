package com.adventofcode.samour.aoc2023.day7

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun calculateGameScore(fname: String, usingWildcards: Boolean) = readResource("day7/$fname").use { file ->
    file.parseCardGame(usingWildcards).totalScore
}
