package com.adventofcode.samour.aoc2023.day7

import java.math.BigInteger

data class CardGame(
    val hands: List<Pair<CardHand, Int>>,
) {

    val totalScore: BigInteger by lazy {
        hands.sortedBy { (hand, _) -> hand }
            .mapIndexed { i, (_, bid) ->
                (i + 1).toBigInteger() * bid.toBigInteger()
            }.reduce { acc, i -> acc + i }
    }
}
