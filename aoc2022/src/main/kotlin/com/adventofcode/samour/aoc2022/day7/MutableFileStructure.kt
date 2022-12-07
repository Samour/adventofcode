package com.adventofcode.samour.aoc2022.day7

sealed class MutableFileStructure {

    abstract val name: String

    data class Directory(
        override val name: String,
        val contents: MutableMap<String, MutableFileStructure>,
    ) : MutableFileStructure() {

        fun pushItem(item: MutableFileStructure) {
            contents.putIfAbsent(item.name, item)
        }
    }

    data class File(
        override val name: String,
        val size: Int,
    ) : MutableFileStructure()
}

fun MutableFileStructure.toFileStructure(): FileStructure = when (this) {
    is MutableFileStructure.Directory -> FileStructure.Directory(
        name = name,
        contents = contents.map { (_, it) -> it.toFileStructure() },
    )
    is MutableFileStructure.File -> FileStructure.File(
        name = name,
        size = size,
    )
}
