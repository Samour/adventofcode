package com.adventofcode.samour.aoc2023.day4

import kotlin.math.pow

data class ScratchCard(
    val cardNumber: Int,
    val winningNumbers: Set<Int>,
    val numbers: List<Int>, // Just in case there are duplicates
) {
    val winningCount: Int by lazy {
        numbers.count(winningNumbers::contains)
    }

    val score: Int by lazy {

        if (winningCount == 0) {
            0
        } else {
            (2.0).pow(winningCount - 1).toInt()
        }
    }
}
