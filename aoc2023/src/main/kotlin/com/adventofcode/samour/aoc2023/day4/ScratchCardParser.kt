package com.adventofcode.samour.aoc2023.day4

import java.io.BufferedReader

fun BufferedReader.parseScratchCards(): List<ScratchCard> =
    readLines().map(::parseScratchCard)

private fun parseScratchCard(line: String): ScratchCard {
    val (cardInfo, numbers) = line.split(":")
    val (winningNumbers, cardNumbers) = numbers.split("|")

    return ScratchCard(
        cardNumber = cardInfo.substring(5).trim().toInt(),
        winningNumbers = winningNumbers.extractNumbers().toSet(),
        numbers = cardNumbers.extractNumbers(),
    )
}

private fun String.extractNumbers() =
    trim()
        .split(" ")
        .map { it.trim() }
        .filterNot { it.isBlank() }
        .map { it.toInt() }
