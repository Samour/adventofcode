package com.adventofcode.samour.aoc2023.day3

data class EngineSchematic(
    val numbers: List<SchematicNumber>,
    val symbols: Map<Pair<Int, Int>, Char>,
)

data class SchematicNumber(
    val value: Int,
    val startPosition: Pair<Int, Int>,
    val numLength: Int,
)
