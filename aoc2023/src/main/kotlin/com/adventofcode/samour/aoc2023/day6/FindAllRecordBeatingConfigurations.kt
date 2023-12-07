package com.adventofcode.samour.aoc2023.day6

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun findAllRecordBeatingConfigurations(fname: String) = readResource("day6/$fname").use { file ->
    file.parseRaceRecords().map { it.countRecordBeatingConfigurations() }
}

fun findRecordBeatingConfigurationForSingleRace(fname: String) = readResource("day6/$fname").use { file ->
    file.parseSingleRaceRecord()
        .countRecordBeatingConfigurations()
}
