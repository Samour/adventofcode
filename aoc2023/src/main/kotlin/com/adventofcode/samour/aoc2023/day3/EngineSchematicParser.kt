package com.adventofcode.samour.aoc2023.day3

import java.io.BufferedReader

fun BufferedReader.parseEngineSchematic(): EngineSchematic {
    val numbers = mutableListOf<SchematicNumber>()
    val symbols = mutableMapOf<Pair<Int, Int>, Char>()
    var numberX = -1
    var numberLength = 0
    var number = 0

    fun resetNumber() {
        numberX = -1
        numberLength = 0
        number = 0
    }

    fun pushNumber(y: Int) {
        if (numberLength > 0) {
            numbers.add(
                SchematicNumber(
                    value = number,
                    startPosition = numberX to y,
                    numLength = numberLength,
                ),
            )
            resetNumber()
        }
    }

    fun processDigitChar(x: Int, c: Char) {
        if (numberX == -1) {
            numberX = x
        }
        number *= 10
        number += c.digitToInt()
        numberLength += 1
    }

    fun processNonDigitChar(x: Int, y: Int, c: Char) {
        pushNumber(y)
        if (c != '.') {
            symbols[x to y] = c
        }
    }

    fun processChar(x: Int, y: Int, c: Char) {
        if (c.isDigit()) {
            processDigitChar(x, c)
        } else {
            processNonDigitChar(x, y, c)
        }
    }

    readLines().forEachIndexed { y, row ->
        row.forEachIndexed { x, c -> processChar(x, y, c) }
        pushNumber(y)
    }

    return EngineSchematic(
        numbers = numbers,
        symbols = symbols,
    )
}
