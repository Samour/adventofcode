package com.adventofcode.samour.aoc2022.day2

sealed class RoundResult(val score: Int) {

    abstract val shape: RPSShape

    fun totalScore() = score + shape.score

    data class Win(override val shape: RPSShape) : RoundResult(6)
    data class Draw(override val shape: RPSShape) : RoundResult(3)
    data class Loss(override val shape: RPSShape) : RoundResult(0)
}
