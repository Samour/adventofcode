package com.adventofcode.samour.aoc2023.day6

import java.math.BigInteger

fun RaceRecord.countRecordBeatingConfigurations(): BigInteger {
    var i = BigInteger.ONE
    var count = BigInteger.ZERO
    while (i < raceTime) {
        if ((raceTime - i) * i > recordDistance) {
            count++
        }
        i++
    }

    return count
}
