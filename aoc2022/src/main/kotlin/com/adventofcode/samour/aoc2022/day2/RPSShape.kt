package com.adventofcode.samour.aoc2022.day2

enum class RPSShape(val score: Int) {

    ROCK(1),
    PAPER(2),
    SCISSORS(3);

    companion object {
        val defeats = mapOf(
            PAPER to ROCK,
            ROCK to SCISSORS,
            SCISSORS to PAPER,
        )

        val defeatedBy = defeats.map { (win, lose) -> lose to win }
            .toMap()
    }
}
