package com.adventofcode.samour.aoc2022.day3

fun scoreItem(item: Char): Int {
    return if (item in 'a'..'z') {
        item - 'a' + 1
    } else if (item in 'A'..'Z') {
        item - 'A' + 27
    } else {
        throw IllegalArgumentException("Cannot score character '$item'")
    }
}
