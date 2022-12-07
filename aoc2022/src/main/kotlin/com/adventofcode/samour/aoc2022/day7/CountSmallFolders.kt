package com.adventofcode.samour.aoc2022.day7

private const val MAX_SIZE = 100_000

fun countSmallFolders(fname: String): Int =
    readFileStructure("day7/$fname").countSmallFolders()

private fun FileStructure.countSmallFolders(): Int = when (this) {
    is FileStructure.Directory -> contents.sumOf { it.countSmallFolders() } +
            (size.takeIf { it <= MAX_SIZE } ?: 0)
    else -> 0
}
