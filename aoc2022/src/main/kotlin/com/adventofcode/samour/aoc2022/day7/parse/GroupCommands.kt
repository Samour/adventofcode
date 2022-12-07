package com.adventofcode.samour.aoc2022.day7.parse

private class CommandExchangeIterator(val source: Iterator<String>) : Iterator<CommandExchange> {

    private var currentCommandLine: String? = null
    private var outputs = mutableListOf<String>()

    override fun hasNext() = source.hasNext() || currentCommandLine != null

    override fun next(): CommandExchange {
        while (source.hasNext()) {
            val line = source.next()
            if (line.startsWith("$ ")) {
                if (currentCommandLine != null) {
                    return CommandExchange(
                        commandLine = currentCommandLine!!,
                        output = outputs,
                    ).also {
                        currentCommandLine = line.substring(2)
                        outputs = mutableListOf()
                    }
                }

                currentCommandLine = line.substring(2)
            } else {
                outputs.add(line)
            }
        }

        return currentCommandLine?.let {
            CommandExchange(
                commandLine = it,
                output = outputs,
            ).also {
                currentCommandLine = null
                outputs = mutableListOf()
            }
        } ?: throw IllegalArgumentException("Could not find any command groupings")
    }
}

fun Sequence<String>.groupCommands(): Sequence<CommandExchange> = Sequence {
    CommandExchangeIterator(iterator())
}
