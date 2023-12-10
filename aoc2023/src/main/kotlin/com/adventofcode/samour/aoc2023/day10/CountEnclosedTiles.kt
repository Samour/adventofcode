package com.adventofcode.samour.aoc2023.day10

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun countEnclosedTiles(fname: String): Int = readResource("day10/$fname").use { file ->
    val map = file.parsePipeSpec().withoutLoops()
    val loopCoords = traceAnimalLoop(map).coordsOnLoop

    map.replaceAnimalByInference().rows.mapIndexed { y, row ->
        val simplifiedRow = mutableListOf<BoundingPipeType>()
        var leadingJoinUp: Boolean? = null

        row.forEachIndexed { x, sectionType ->
            if (!loopCoords.contains(x to y) || sectionType == SectionType.NONE) {
                simplifiedRow.add(BoundingPipeType.EMPTY)
            } else if (sectionType == SectionType.PIPE_VERTICAL) {
                simplifiedRow.add(BoundingPipeType.PIPE_BOUNDING)
            } else if (sectionType == SectionType.PIPE_JOIN_RU) {
                leadingJoinUp = true
            } else if (sectionType == SectionType.PIPE_JOIN_RD) {
                leadingJoinUp = false
            } else if (sectionType == SectionType.PIPE_JOIN_LU) {
                if (leadingJoinUp!!) {
                    simplifiedRow.add(BoundingPipeType.PIPE_NON_BOUNDING)
                } else {
                    simplifiedRow.add(BoundingPipeType.PIPE_BOUNDING)
                }
            } else if (sectionType == SectionType.PIPE_JOIN_LD) {
                if (leadingJoinUp!!) {
                    simplifiedRow.add(BoundingPipeType.PIPE_BOUNDING)
                } else {
                    simplifiedRow.add(BoundingPipeType.PIPE_NON_BOUNDING)
                }
            }
        }

        var inLoop = false
        simplifiedRow.count {
            when (it) {
                BoundingPipeType.PIPE_BOUNDING -> {
                    inLoop = !inLoop
                    false
                }
                BoundingPipeType.PIPE_NON_BOUNDING -> false
                BoundingPipeType.EMPTY -> inLoop
            }
        }
    }.sum()
}

private enum class BoundingPipeType {
    PIPE_BOUNDING,
    PIPE_NON_BOUNDING,
    EMPTY,
}
