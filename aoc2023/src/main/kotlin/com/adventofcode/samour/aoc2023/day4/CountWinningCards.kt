package com.adventofcode.samour.aoc2023.day4

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun countWinningCards(fname: String) = readResource("day4/$fname").use { file ->
    file.parseScratchCards()
}

fun countCascadedCopies(fname: String) = readResource("day4/$fname").use { file ->
    val allCards = file.parseScratchCards()

    val cardCopies = (1..allCards.size).associateWith { 1 }.toMutableMap()
    fun updateCopyCount(cardIdx: Int) {
        (1..allCards[cardIdx - 1].winningCount).forEach {
            val nextIdx = cardIdx + it
            cardCopies[nextIdx] = cardCopies[nextIdx]!! + 1
            updateCopyCount(nextIdx)
        }
    }
    cardCopies.keys.forEach { updateCopyCount(it) }

    cardCopies
}
