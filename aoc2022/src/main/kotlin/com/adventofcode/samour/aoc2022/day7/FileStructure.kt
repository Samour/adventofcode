package com.adventofcode.samour.aoc2022.day7

sealed class FileStructure {

    abstract val size: Int

    data class Directory(
        val name: String,
        val contents: List<FileStructure>,
    ) : FileStructure() {
        override val size by lazy {
            contents.sumOf { it.size }
        }
    }

    data class File(
        val name: String,
        override val size: Int,
    ) : FileStructure()
}
