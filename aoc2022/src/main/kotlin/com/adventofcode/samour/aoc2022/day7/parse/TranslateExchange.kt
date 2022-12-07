package com.adventofcode.samour.aoc2022.day7.parse

import com.adventofcode.samour.aoc2022.day7.DirectoryMember
import com.adventofcode.samour.aoc2022.day7.TerminalCommand

fun translateExchange(exchange: CommandExchange): TerminalCommand {
    val commandParts = exchange.commandLine.split(" ")

    return when (commandParts[0]) {
        "ls" -> translateListExchange(commandParts, exchange.output)
        "cd" -> translateChangeDirectory(commandParts, exchange.output)
        else -> throw IllegalArgumentException("Unexpected command '${commandParts[0]}'")
    }
}

private fun translateListExchange(commandParts: List<String>, members: List<String>): TerminalCommand.ListDirectory {
    if (commandParts.size != 1) {
        throw IllegalArgumentException("Command 'ls' does not accept arguments")
    }

    return TerminalCommand.ListDirectory(
        members = members.map { translateDirectoryMember(it) },
    )
}

private fun translateDirectoryMember(line: String): DirectoryMember {
    val parts = line.split(" ")
    if (parts.size != 2) {
        throw IllegalArgumentException("Wrong number of elements in directory member '$line'")
    }

    return if (parts[0] == "dir") {
        DirectoryMember.Directory(parts[1])
    } else {
        DirectoryMember.File(
            name = parts[1],
            size = parts[0].toInt(),
        )
    }
}

private fun translateChangeDirectory(
    commandParts: List<String>,
    output: List<String>
): TerminalCommand.ChangeDirectory {
    if (commandParts.size != 2) {
        throw IllegalArgumentException("Command 'cd' expects exactly 1 argument")
    }
    if (output.isNotEmpty()) {
        throw IllegalArgumentException("Command 'cd' should not generate any output")
    }

    return TerminalCommand.ChangeDirectory(commandParts[1])
}
