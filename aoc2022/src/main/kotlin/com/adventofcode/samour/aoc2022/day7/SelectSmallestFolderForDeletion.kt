package com.adventofcode.samour.aoc2022.day7

private const val TOTAL_SPACE = 70_000_000
private const val FREE_REQUIRED = 30_000_000

fun selectSmallestFolderForDeletion(fname: String): Int {
    val fileStructure = readFileStructure("day7/$fname")
    val removalTargetSize = FREE_REQUIRED - (TOTAL_SPACE - fileStructure.size)
    if (removalTargetSize <= 0) {
        return 0
    }

    return fileStructure.allDirectories()
        .map { it.size }
        .filter { it >= removalTargetSize }
        .min()
}

private fun FileStructure.allDirectories(): List<FileStructure.Directory> = when (this) {
    is FileStructure.Directory -> listOf(this) + this.contents.flatMap { it.allDirectories() }
    else -> emptyList()
}
