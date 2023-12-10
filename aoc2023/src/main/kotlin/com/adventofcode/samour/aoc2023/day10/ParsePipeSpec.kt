package com.adventofcode.samour.aoc2023.day10

import java.io.BufferedReader

fun BufferedReader.parsePipeSpec(): PipeSpec = PipeSpec(
    rows = readLines().map { line ->
        line.trim().toCharArray().map {
            when (it) {
                '|' -> SectionType.PIPE_VERTICAL
                '-' -> SectionType.PIPE_HORIZONTAL
                'F' -> SectionType.PIPE_JOIN_RD
                '7' -> SectionType.PIPE_JOIN_LD
                'L' -> SectionType.PIPE_JOIN_RU
                'J' -> SectionType.PIPE_JOIN_LU
                '.' -> SectionType.NONE
                'S' -> SectionType.ANIMAL
                else -> throw IllegalArgumentException("Unknown character: $it")
            }
        }
    },
)
