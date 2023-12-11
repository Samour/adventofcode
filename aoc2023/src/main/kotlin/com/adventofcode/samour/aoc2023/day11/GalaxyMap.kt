package com.adventofcode.samour.aoc2023.day11

data class GalaxyMap(
    val rows: List<List<GalaxyState>>,
) {

    fun expandEmptySpaces(): GalaxyMap {
        val emptyColumns = (0 until rows.first().size).filter { x ->
            rows.all { it[x] == GalaxyState.NONE }
        }.toSet()

        val resultRows = mutableListOf<List<GalaxyState>>()
        rows.forEach { row ->
            val resultRow = mutableListOf<GalaxyState>()
            var isEmptyRow = true
            row.forEachIndexed { x, galaxyState ->
                isEmptyRow = isEmptyRow && galaxyState == GalaxyState.NONE
                resultRow.add(galaxyState)
                if (emptyColumns.contains(x)) {
                    resultRow.add(GalaxyState.NONE)
                }
            }
            resultRows.add(resultRow)
            if (isEmptyRow) {
                resultRows.add(resultRow)
            }
        }

        return GalaxyMap(resultRows)
    }
}

enum class GalaxyState {
    PRESENT,
    NONE,
}
