package com.adventofcode.samour.aoc2022.day7

import com.adventofcode.samour.aoc2022.day7.parse.groupCommands
import com.adventofcode.samour.aoc2022.day7.parse.translateExchange
import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun readFileStructure(fname: String): FileStructure {
    val commandInterpreter = CommandInterpreter()
    readResource(fname).lineSequence()
        .groupCommands()
        .map { translateExchange(it) }
        .forEach(commandInterpreter::processCommand)

    return commandInterpreter.fileStructure
        ?: throw IllegalArgumentException("Failed to interpret file structure from commands")
}
