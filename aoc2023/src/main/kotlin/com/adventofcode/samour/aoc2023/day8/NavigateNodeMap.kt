package com.adventofcode.samour.aoc2023.day8

fun NodeMap.navigate(simulNavigate: Boolean): Int {
    var instruction = 0
    var node = if (simulNavigate) {
        nodes.keys.filter { it.endsWith("A") }
    } else {
        listOf("AAA")
    }
    var steps = 0

    while (!node.isAtEnd(simulNavigate)) {
        steps++
        node = node.map {
            when (instructions[instruction]) {
                'L' -> nodes[it]!!.first
                'R' -> nodes[it]!!.second
                else -> throw Exception("Unknown instruction: ${instructions[instruction]}")
            }
        }
        instruction = (instruction + 1) % instructions.size
    }

    return steps
}

private fun List<String>.isAtEnd(simulNavigate: Boolean): Boolean = if (simulNavigate) {
    all { it.endsWith("Z") }
} else {
    first() == "ZZZ"
}
