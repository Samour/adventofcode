package com.adventofcode.samour.aoc2023.day10

data class PipeSpec(
    val rows: List<List<SectionType>>,
) {

    fun adjacentSections(x: Int, y: Int): List<ConnectionSpec> = PositionSpec(
        x = x,
        y = y,
        sectionType = rows[y][x],
    ).let { position ->
        position.adjacentCoordinates().mapNotNull { (i, j) ->
            rows.getOrNull(j)?.getOrNull(i)?.let {
                PositionSpec(
                    x = i,
                    y = j,
                    sectionType = it,
                )
            }
        }.map {
            ConnectionSpec(
                section = it,
                connects = when (it.sectionType) {
                    SectionType.ANIMAL -> PartiallySpecified.MAYBE
                    SectionType.NONE -> PartiallySpecified.FALSE
                    else -> if (it.adjacentCoordinates().contains(x to y)) {
                        PartiallySpecified.TRUE
                    } else {
                        PartiallySpecified.FALSE
                    }
                },
            )
        }
    }

    fun withoutLoops(): PipeSpec {
        val next = removeDeadEndSections()
        return if (next == this) {
            this
        } else {
            next.withoutLoops()
        }
    }

    private fun removeDeadEndSections(): PipeSpec {
        return PipeSpec(
            rows = rows.mapIndexed { y, row ->
                row.mapIndexed { x, sectionType ->
                    when (sectionType) {
                        SectionType.NONE, SectionType.ANIMAL -> sectionType

                        else -> sectionType.takeIf { _ ->
                            adjacentSections(x, y).let {
                                it.size == 2 && it.all { c -> c.connects != PartiallySpecified.FALSE }
                            }
                        } ?: SectionType.NONE
                    }
                }
            },
        )
    }

    fun debugRender(): String = rows.joinToString("\n") { row ->
        row.joinToString("") { "${it.debugRender()}" }
    }

    fun inferSectionTypeOfAnimal(): SectionType {
        val (animalX, animalY) = rows.flatMapIndexed { y, row ->
            row.mapIndexed { x, sectionType ->
                PositionSpec(
                    x = x,
                    y = y,
                    sectionType = sectionType,
                )
            }
        }.first { it.sectionType == SectionType.ANIMAL }

        val connectedPositions = listOf(
            animalX - 1 to animalY,
            animalX + 1 to animalY,
            animalX to animalY - 1,
            animalX to animalY + 1,
        ).filter { (x, y) ->
            rows.getOrNull(y)?.getOrNull(x).let {
                it != null && PositionSpec(x, y, it).adjacentCoordinates().contains(animalX to animalY)
            }
        }.toSet()

        if (connectedPositions == setOf(animalX - 1 to animalY, animalX + 1 to animalY)) {
            return SectionType.PIPE_HORIZONTAL
        } else if (connectedPositions == setOf(animalX to animalY - 1, animalX to animalY + 1)) {
            return SectionType.PIPE_VERTICAL
        } else if (connectedPositions == setOf(animalX + 1 to animalY, animalX to animalY + 1)) {
            return SectionType.PIPE_JOIN_RD
        } else if (connectedPositions == setOf(animalX - 1 to animalY, animalX to animalY + 1)) {
            return SectionType.PIPE_JOIN_LD
        } else if (connectedPositions == setOf(animalX + 1 to animalY, animalX to animalY - 1)) {
            return SectionType.PIPE_JOIN_RU
        } else if (connectedPositions == setOf(animalX - 1 to animalY, animalX to animalY - 1)) {
            return SectionType.PIPE_JOIN_LU
        } else {
            throw IllegalStateException("Cannot determine pipe type of animal position")
        }
    }

    fun replaceAnimalByInference(): PipeSpec = PipeSpec(
        rows = rows.mapIndexed { y, row ->
            row.mapIndexed { x, sectionType ->
                if (sectionType == SectionType.ANIMAL) {
                    inferSectionTypeOfAnimal()
                } else {
                    sectionType
                }
            }
        }
    )
}

enum class SectionType {
    PIPE_VERTICAL,
    PIPE_HORIZONTAL,

    PIPE_JOIN_RD,
    PIPE_JOIN_LD,
    PIPE_JOIN_RU,
    PIPE_JOIN_LU,

    NONE,
    ANIMAL;

    fun debugRender(): Char = when (this) {
        PIPE_VERTICAL -> '|'
        PIPE_HORIZONTAL -> '-'
        PIPE_JOIN_RD -> 'F'
        PIPE_JOIN_LD -> '7'
        PIPE_JOIN_RU -> 'L'
        PIPE_JOIN_LU -> 'J'
        NONE -> '.'
        ANIMAL -> 'S'
    }
}

data class PositionSpec(
    val x: Int,
    val y: Int,
    val sectionType: SectionType,
) {
    fun adjacentCoordinates(): Set<Pair<Int, Int>> = when (sectionType) {
        SectionType.PIPE_VERTICAL -> setOfNotNull(
            x to y - 1,
            x to y + 1,
        )

        SectionType.PIPE_HORIZONTAL -> setOfNotNull(
            x - 1 to y,
            x + 1 to y,
        )

        SectionType.PIPE_JOIN_RD -> setOfNotNull(
            x + 1 to y,
            x to y + 1,
        )

        SectionType.PIPE_JOIN_LD -> setOfNotNull(
            x - 1 to y,
            x to y + 1,
        )

        SectionType.PIPE_JOIN_RU -> setOfNotNull(
            x + 1 to y,
            x to y - 1,
        )

        SectionType.PIPE_JOIN_LU -> setOfNotNull(
            x - 1 to y,
            x to y - 1,
        )

        else -> emptySet()
    }
}

data class ConnectionSpec(
    val section: PositionSpec,
    val connects: PartiallySpecified,
)

enum class PartiallySpecified {
    TRUE,
    FALSE,
    MAYBE
}
