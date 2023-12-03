package com.adventofcode.samour.aoc2023.day3

fun EngineSchematic.findNumbersAdjacentToAnySymbol(): List<Int> = numbers.filter { number ->
    val (x, y) = number.startPosition
    computeAdjacentPositions(x, y, number.numLength).any {
        symbols[it] != null
    }
}.map { it.value }

fun EngineSchematic.findNumbersAdjacentToPosition(x: Int, y: Int): List<Int> = numbers.filter { number ->
    val (i, j) = number.startPosition
    computeAdjacentPositions(i, j, number.numLength).contains(x to y)
}.map { it.value }

fun computeAdjacentPositions(x: Int, y: Int, length: Int): List<Pair<Int, Int>> {
    return listOf(
        x - 1 to y,
        x + length to y,
    ) + (-1..length).flatMap {
        listOf(
            x + it to y - 1,
            x + it to y + 1,
        )
    }
}
