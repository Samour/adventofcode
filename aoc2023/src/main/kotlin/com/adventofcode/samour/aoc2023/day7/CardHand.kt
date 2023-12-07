package com.adventofcode.samour.aoc2023.day7

data class CardHand(
    val cards: List<Char>,
) : Comparable<CardHand> {

    private val cardsByFrequency: List<Pair<Char, Int>> by lazy {
        val counts = mutableMapOf<Char, Int>()
        cards.forEach { c ->
            counts[c] = (counts[c] ?: 0) + 1
        }
        counts.entries
            .sortedByDescending { (_, count) -> count }
            .map { (c, count) -> c to count }
    }

    val handType: HandType by lazy {
        if (cardsByFrequency.first().second == 5) {
            HandType.FIVE_OF_KIND
        } else if (cardsByFrequency.first().second == 4) {
            HandType.FOUR_OF_KIND
        } else if (cardsByFrequency.first().second == 3 && cardsByFrequency[1].second == 2) {
            HandType.FULL_HOUSE
        } else if (cardsByFrequency.first().second == 3) {
            HandType.THREE_OF_KIND
        } else if (cardsByFrequency.first().second == 2 && cardsByFrequency[1].second == 2) {
            HandType.TWO_PAIR
        } else if (cardsByFrequency.first().second == 2) {
            HandType.PAIR
        } else {
            HandType.HIGH_CARD
        }
    }

    override fun compareTo(other: CardHand): Int {
        if (other.handType != handType) {
            return handType.strength - other.handType.strength
        }

        cards.forEachIndexed { i, c ->
            val cardScore = cardStrength(c)
            val otherCardScore = cardStrength(other.cards[i])
            if (cardScore != otherCardScore) {
                return cardScore - otherCardScore
            }
        }

        return 0
    }
}

enum class HandType(val strength: Int) {
    FIVE_OF_KIND(7),
    FOUR_OF_KIND(6),
    FULL_HOUSE(5),
    THREE_OF_KIND(4),
    TWO_PAIR(3),
    PAIR(2),
    HIGH_CARD(1),
}

private fun cardStrength(card: Char): Int =
    when (card) {
        'A' -> 14
        'K' -> 13
        'Q' -> 12
        'J' -> 11
        'T' -> 10
        else -> card.digitToInt()
    }
