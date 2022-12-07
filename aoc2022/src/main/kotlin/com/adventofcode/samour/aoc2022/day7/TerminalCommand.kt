package com.adventofcode.samour.aoc2022.day7

sealed class DirectoryMember {

    data class Directory(val name: String) : DirectoryMember()

    data class File(val name: String, val size: Int) : DirectoryMember()
}

sealed class TerminalCommand {

    data class ChangeDirectory(val directoryName: String) : TerminalCommand()

    data class ListDirectory(val members: List<DirectoryMember>) : TerminalCommand()
}
