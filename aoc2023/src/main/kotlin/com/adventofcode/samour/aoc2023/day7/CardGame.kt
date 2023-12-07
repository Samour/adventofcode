package com.adventofcode.samour.aoc2023.day7

data class CardGame(
    val hands: List<Pair<CardHand, Int>>,
) {

    val totalScore: Int by lazy {
        hands.sortedBy { (hand, _) -> hand }
            .mapIndexed { i, (_, bid) ->
                (i + 1) * bid
            }.sum()
    }
}
