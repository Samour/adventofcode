package com.adventofcode.samour.aoc2023.day7

data class CardHand(
    val cards: List<Char>,
    val usingWildcards: Boolean,
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

    private val jokerCount: Int by lazy {
        if (usingWildcards) {
            cards.count { it == 'J' }
        } else {
            0
        }
    }

    private val mostFrequent: Pair<Char, Int> by lazy {
        if (cardsByFrequency.size > 1 && usingWildcards && cardsByFrequency.first().first == 'J') {
            cardsByFrequency[1]
        } else {
            cardsByFrequency.first()
        }
    }

    private val secondMostFrequent: Pair<Char, Int> by lazy {
        if (cardsByFrequency.size == 1) {
            'X' to 0
        } else if (cardsByFrequency.size > 2 && usingWildcards && cardsByFrequency[1].first == 'J') {
            cardsByFrequency[2]
        } else {
            cardsByFrequency[1]
        }
    }

    val handType: HandType by lazy {
        if (isFiveOfKind()) {
            HandType.FIVE_OF_KIND
        } else if (isFourOfKind()) {
            HandType.FOUR_OF_KIND
        } else if (isFullHouse()) {
            HandType.FULL_HOUSE
        } else if (isThreeOfKind()) {
            HandType.THREE_OF_KIND
        } else if (isTwoPair()) {
            HandType.TWO_PAIR
        } else if (isPair()) {
            HandType.PAIR
        } else {
            HandType.HIGH_CARD
        }
    }

    private fun isFiveOfKind(): Boolean {
        return mostFrequent.second + jokerCount >= 5
    }

    private fun isFourOfKind(): Boolean {
        return mostFrequent.second + jokerCount >= 4
    }

    private fun isFullHouse(): Boolean {
        if (!isThreeOfKind()) {
            return false
        }

        var remainingJokers = jokerCount
        if (mostFrequent.second < 3) {
            remainingJokers -= 3 - mostFrequent.second
        }
        return secondMostFrequent.second + remainingJokers >= 2
    }

    private fun isThreeOfKind(): Boolean {
        return mostFrequent.second + jokerCount >= 3
    }

    private fun isTwoPair(): Boolean {
        if (!isPair()) {
            return false
        }

        var remainingJokers = jokerCount
        if (mostFrequent.second < 2) {
            remainingJokers -= 2 - mostFrequent.second
        }
        return secondMostFrequent.second + remainingJokers >= 2
    }

    private fun isPair(): Boolean {
        return mostFrequent.second + jokerCount >= 2
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

    private fun cardStrength(card: Char): Int =
        when (card) {
            'A' -> 14
            'K' -> 13
            'Q' -> 12
            'J' -> if (usingWildcards) 1 else 11
            'T' -> 10
            else -> card.digitToInt()
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
