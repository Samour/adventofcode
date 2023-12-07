package com.adventofcode.samour.aoc2023.day7

import java.io.BufferedReader

fun BufferedReader.parseCardGame(usingWildcards: Boolean): CardGame = CardGame(
    hands = readLines().map { line ->
        val (cards, bid) = line.split(" ")
        CardHand(
            cards = cards.toCharArray().toList(),
            usingWildcards = usingWildcards,
        ) to bid.toInt()
    },
)
