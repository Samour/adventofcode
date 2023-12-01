package com.adventofcode.samour.aoc2023.day1

import java.io.BufferedReader

data class Digits(var digits: List<Int>) {
    fun toTwoDigitNum() = digits.first() * 10 + digits.last()
}

fun BufferedReader.readDigits() = readLines().map {
    Digits(it.filterDigits())
}

private fun String.filterDigits(): List<Int> =
        toCharArray().filter { it.isDigit() }
                .map { it.digitToInt() }
                .toList()
