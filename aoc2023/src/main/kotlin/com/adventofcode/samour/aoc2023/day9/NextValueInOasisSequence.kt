package com.adventofcode.samour.aoc2023.day9

fun nextValueInOasisSequence(sequence: List<Int>, extrapolateBackwards: Boolean): Int {
    val differences = differencesSequences(listOf(sequence)).toMutableList()
    var lastDifference = differences.removeLast().attachNewValue(0, !extrapolateBackwards)
    while (differences.isNotEmpty()) {
        val currentDifference = differences.removeLast()
        lastDifference = currentDifference.attachNewValue(
            if (extrapolateBackwards) {
                currentDifference.first() - lastDifference.first()
            } else {
                currentDifference.last() + lastDifference.last()
            },
            !extrapolateBackwards,
        )
    }

    return if (extrapolateBackwards) {
        lastDifference.first()
    } else {
        lastDifference.last()
    }
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

private fun List<Int>.attachNewValue(value: Int, addToEnd: Boolean): List<Int> = if (addToEnd) {
    plusElement(value)
} else {
    listOf(value) + this
}
