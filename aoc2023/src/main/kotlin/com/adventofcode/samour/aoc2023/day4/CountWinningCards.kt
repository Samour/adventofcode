package com.adventofcode.samour.aoc2023.day4

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun countWinningCards(fname: String) = readResource("day4/$fname").use { file ->
    file.parseScratchCards()
}

fun countCascadedCopies(fname: String) = readResource("day4/$fname").use { file ->
    val allCards = file.parseScratchCards()

    val copiesToCreateCache = mutableMapOf<Int, Map<Int, Int>>()
    fun copiesToCreate(cardIdx: Int): Map<Int, Int> {
        println("copiesToCreate($cardIdx)")
        copiesToCreateCache[cardIdx]?.let { return it }

        val copies = mutableMapOf<Int, Int>()
        if (cardIdx == allCards.size + 1 || allCards[cardIdx - 1].winningCount == 0) {
            copiesToCreateCache[cardIdx] = copies
            return copies
        }

        (1..allCards[cardIdx - 1].winningCount).forEach {
            copies[cardIdx + it] = copies.getOrDefault(cardIdx + it, 0) + 1
            copiesToCreate(cardIdx + it).forEach { (copyIdx, copyCount) ->
                copies[copyIdx] = copies.getOrDefault(copyIdx, 0) + copyCount
            }
        }

        copiesToCreateCache[cardIdx] = copies
        return copies
    }

    val cardCopies = (1..allCards.size).associateWith { 1 }.toMutableMap()
    cardCopies.keys.forEach { cardIdx ->
        copiesToCreate(cardIdx).forEach { (copyIdx, copyCount) ->
            cardCopies[copyIdx] = cardCopies[copyIdx]!! + copyCount
        }
    }
    cardCopies
}
