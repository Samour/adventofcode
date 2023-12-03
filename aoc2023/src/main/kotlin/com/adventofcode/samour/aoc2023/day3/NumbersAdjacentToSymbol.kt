package com.adventofcode.samour.aoc2023.day3

fun EngineSchematic.findNumbersAdjacentToSymbol(): List<Int> = numbers.filter { number ->
    val (x, y) = number.startPosition
    computeAdjacentPositions(x, y, number.numLength).any {
        symbols[it] != null
    }
}.map { it.value }

private fun computeAdjacentPositions(x: Int, y: Int, length: Int): List<Pair<Int, Int>> {
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
