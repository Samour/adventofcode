package com.adventofcode.samour.aoc2023.day6

fun RaceRecord.countRecordBeatingConfigurations(): Int =
    (1 until raceTime).count { i ->
        (raceTime - i) * i > recordDistance
    }
