package com.adventofcode.samour.aoc2023.day4

import kotlin.math.pow

data class ScratchCard(
    val cardNumber: Int,
    val winningNumbers: Set<Int>,
    val numbers: List<Int>, // Just in case there are duplicates
) {
    val score: Int by lazy {
        val winningCount = numbers.count(winningNumbers::contains)
        if (winningCount == 0) {
            0
        } else {
            (2.0).pow(winningCount - 1).toInt()
        }
    }
}
