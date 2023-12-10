package com.adventofcode.samour.aoc2023.day10

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun findDistanceFromStart(fname: String) = readResource("day10/$fname").use { file ->
    findFurthestDistanceFromStart(file.parsePipeSpec().withoutLoops())
}

private fun findFurthestDistanceFromStart(map: PipeSpec): Int {
    val (startX, startY) = map.rows.flatMapIndexed { y, row ->
        row.mapIndexed { x, sectionType ->
            PositionSpec(
                x = x,
                y = y,
                sectionType = sectionType,
            )
        }
    }.first { it.sectionType == SectionType.ANIMAL }

    var steps = 0
    val encounteredPositions = mutableSetOf(startX to startY)
    var stepFrom = setOf(
        startX - 1 to startY,
        startX + 1 to startY,
        startX to startY - 1,
        startX to startY + 1,
    ).filter { (x, y) ->
        map.rows.getOrNull(y)?.getOrNull(x).let {
            it != null && PositionSpec(x, y, it).adjacentCoordinates().contains(startX to startY)
        }
    }.toSet()

    while (stepFrom.isNotEmpty()) {
        encounteredPositions.addAll(stepFrom)
        stepFrom = stepFrom.flatMap { (x, y) ->
            map.adjacentSections(x, y).filter {
                it.section.sectionType != SectionType.ANIMAL && it.section.sectionType != SectionType.NONE
                    && it.connects == PartiallySpecified.TRUE
            }.map { it.section.x to it.section.y }
        }.filter { !encounteredPositions.contains(it) }
            .toSet()
        steps++
    }

    return steps
}
