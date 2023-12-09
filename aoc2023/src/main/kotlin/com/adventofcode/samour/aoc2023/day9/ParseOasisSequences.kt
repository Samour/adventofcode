package com.adventofcode.samour.aoc2023.day9

import java.io.BufferedReader

fun BufferedReader.parseOasisSequences(): List<List<Int>> = readLines().map { line ->
    line.split(" ").mapNotNull { s ->
        s.trim().takeIf { it.isNotBlank() }
            ?.toInt()
    }
}
