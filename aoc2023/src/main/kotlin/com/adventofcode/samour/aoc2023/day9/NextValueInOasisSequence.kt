package com.adventofcode.samour.aoc2023.day9

fun nextValueInOasisSequence(sequence: List<Int>): Int {
    val differences = differencesSequences(listOf(sequence)).toMutableList()
    var lastDifference = differences.removeLast().plusElement(0)
    while (differences.isNotEmpty()) {
        val currentDifference = differences.removeLast()
        lastDifference = currentDifference.plusElement(
            currentDifference.last() + lastDifference.last()
        )
    }

    return lastDifference.last()
}

private tailrec fun differencesSequences(priorSequences: List<List<Int>>): List<List<Int>> {
    if (priorSequences.last().all { it == 0 }) {
        return priorSequences
    }

    return differencesSequences(
        priorSequences.plusElement(
            priorSequences.last().let { seq ->
                (1 until seq.size).map { seq[it] - seq[it - 1] }
            },
        ),
    )
}
