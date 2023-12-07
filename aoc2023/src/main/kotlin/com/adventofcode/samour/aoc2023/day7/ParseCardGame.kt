package com.adventofcode.samour.aoc2023.day7

import java.io.BufferedReader

fun BufferedReader.parseCardGame(): CardGame = CardGame(
    hands = readLines().map { line ->
        val (cards, bid) = line.split(" ")
        CardHand(
            cards = cards.toCharArray().toList(),
        ) to bid.toInt()
    },
)
