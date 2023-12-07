package com.adventofcode.samour.aoc2023.day6

import java.io.BufferedReader

fun BufferedReader.parseRaceRecords(): List<RaceRecord> {
    val times = readLine().substring(5).split(" ")
        .map { it.trim() }
        .filter { it.isNotBlank() }
        .map { it.toBigInteger() }
    val records = readLine().substring(9).split(" ")
        .map { it.trim() }
        .filter { it.isNotBlank() }
        .map { it.toBigInteger() }

    return times.zip(records).map { (time, record) ->
        RaceRecord(
            raceTime = time,
            recordDistance = record,
        )
    }
}

fun BufferedReader.parseSingleRaceRecord(): RaceRecord {
    val time = readLine().substring(5).split(" ")
        .map { it.trim() }
        .filter { it.isNotBlank() }
        .joinToString("")
        .toBigInteger()
    val record = readLine().substring(9).split(" ")
        .map { it.trim() }
        .filter { it.isNotBlank() }
        .joinToString("")
        .toBigInteger()

    return RaceRecord(
        raceTime = time,
        recordDistance = record,
    )
}
