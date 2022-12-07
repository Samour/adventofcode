package com.adventofcode.samour.aoc2022.day7

class CommandInterpreter {

    private var directoryContext: MutableList<MutableFileStructure.Directory>? = null

    val fileStructure: FileStructure?
        get() = directoryContext?.first()?.toFileStructure()

    fun processCommand(command: TerminalCommand) {
        if (directoryContext == null) {
            return initialiseDirectoryContext(command)
        }

        when (command) {
            is TerminalCommand.ListDirectory -> pushDirectoryContents(command.members)
            is TerminalCommand.ChangeDirectory -> changeDirectory(command.directoryName)
        }
    }

    private fun initialiseDirectoryContext(command: TerminalCommand) {
        if (command != TerminalCommand.ChangeDirectory("/")) {
            throw IllegalArgumentException("First command must be 'cd /'")
        }

        directoryContext = mutableListOf(
            MutableFileStructure.Directory(name = "", contents = mutableMapOf())
        )
    }

    private fun pushDirectoryContents(contents: List<DirectoryMember>) {
        contents.forEach {
            directoryContext!!.last().pushItem(it.toMutableFileStructure())
        }
    }

    private fun changeDirectory(target: String) {
        if (target.startsWith("/")) {
            throw IllegalArgumentException("Absolute path changes are not supported")
        } else if (target.contains("/")) {
            throw IllegalArgumentException("Cannot traverse more than 1 directory at a time")
        }

        if (target == ".") {
            return // No-op; change to current directory
        }

        if (target == "..") {
            if (directoryContext!!.size == 1) {
                throw IllegalArgumentException("Context is already at root directory; cannot change upwards")
            }
            directoryContext!!.removeAt(directoryContext!!.lastIndex)
            return
        }

        val targetNode = directoryContext!!.last().contents[target]
            ?: throw IllegalArgumentException("Could not find directory '$target'")
        if (targetNode !is MutableFileStructure.Directory) {
            throw IllegalArgumentException("'$target' is not a directory")
        }
        directoryContext!!.add(targetNode)
    }
}

private fun DirectoryMember.toMutableFileStructure(): MutableFileStructure = when (this) {
    is DirectoryMember.Directory -> MutableFileStructure.Directory(
        name = name,
        contents = mutableMapOf(),
    )
    is DirectoryMember.File -> MutableFileStructure.File(
        name = name,
        size = size,
    )
}
