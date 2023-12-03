package com.adventofcode.samour.aoc2023.day3

import java.io.BufferedReader

fun BufferedReader.parseEngineSchematic(): EngineSchematic {
    return EngineSchematicParser(this).parse()
}

private class EngineSchematicParser(private val source: BufferedReader) {

    private val numbers = mutableListOf<SchematicNumber>()
    private val symbols = mutableMapOf<Pair<Int, Int>, Char>()
    private var numberX = -1
    private var numberLength = 0
    private var number = 0

    fun parse(): EngineSchematic {
        resetNumber()
        source.readLines().forEachIndexed { y, row ->
            resetNumber()
            row.forEachIndexed { x, c ->
                if (c.isDigit()) {
                    if (numberX == -1) {
                        numberX = x
                    }
                    number *= 10
                    number += c.digitToInt()
                    numberLength += 1
                } else {
                    if (numberLength > 0) {
                        pushNumber(y)
                        resetNumber()
                    }
                    if (c != '.') {
                        symbols[x to y] = c
                    }
                }
            }

            pushNumber(y)
        }

        return EngineSchematic(
            numbers = numbers,
            symbols = symbols,
        )
    }

    private fun resetNumber() {
        numberX = -1
        numberLength = 0
        number = 0
    }

    private fun pushNumber(y: Int) {
        if (numberLength > 0) {
            numbers.add(
                SchematicNumber(
                    value = number,
                    startPosition = numberX to y,
                    numLength = numberLength,
                )
            )
        }
    }
}
