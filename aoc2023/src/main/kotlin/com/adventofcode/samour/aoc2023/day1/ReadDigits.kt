package com.adventofcode.samour.aoc2023.day1

import java.io.BufferedReader

typealias DigitParser = (String) -> List<Int>

data class Digits(var digits: List<Int>) {
    fun toTwoDigitNum() = digits.first() * 10 + digits.last()
}

fun BufferedReader.readDigits(digitParser: DigitParser) = readLines().map {
    Digits(digitParser(it))
}

fun filterOnlyDigits(line: String): List<Int> =
        line.toCharArray()
                .filter { it.isDigit() }
                .map { it.digitToInt() }
                .toList()
