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

private val numberWordMap = mapOf(
    "one" to 1,
    "two" to 2,
    "three" to 3,
    "four" to 4,
    "five" to 5,
    "six" to 6,
    "seven" to 7,
    "eight" to 8,
    "nine" to 9,
    "zero" to 0
)
private val numberWordRegex = numberWordMap.keys.joinToString(separator = "|").toRegex()

fun filterDigitsAndWords(line: String): List<Int> {
    return line.indices.mapNotNull { i ->
        if (line[i].isDigit()) {
            line[i].digitToInt()
        } else {
            numberWordRegex.matchAt(line, i)
                ?.value
                ?.let { numberWordMap[it] }
        }
    }
}
