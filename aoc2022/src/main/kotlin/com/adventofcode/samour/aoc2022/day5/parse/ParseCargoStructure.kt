package com.adventofcode.samour.aoc2022.day5.parse

import com.adventofcode.samour.aoc2022.day5.CargoStructure

private val containerPattern = Regex("\\[([A-Z])\\]")

fun parseCargoStructure(cargoLines: List<String>, columns: String): CargoStructure {
    val width = divideColumns(columns).size
    return cargoLines.map { divideColumns(it) }
        .foldIndexed(
            CargoStructure(
                stacks = (1..width).map { listOf() }
            )
        ) { i, structure, row ->
            if (row.size > width) {
                throw IllegalArgumentException(
                    "Row $i of cargo structure has incorrect width. Is ${row.size} when it should be no more than $width"
                )
            }
            structure.pushRow(row)
        }
}

private fun divideColumns(row: String): List<String> =
    (0..((row.length) / 4)).map {
        row.substring(it * 4, minOf(it * 4 + 3, row.length))
    }


private fun CargoStructure.pushRow(row: List<String>): CargoStructure = CargoStructure(
    stacks = stacks.mapIndexed { i, containers ->
        row.getOrNull(i)?.let { parseContainer(it) }
            ?.let {
                containers + listOf(it)
            } ?: containers
    }
)

private fun parseContainer(token: String): Char? =
    containerPattern.matchEntire(token)?.groupValues?.get(1)?.get(0)
