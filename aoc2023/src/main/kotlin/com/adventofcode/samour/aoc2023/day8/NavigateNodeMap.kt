package com.adventofcode.samour.aoc2023.day8

fun NodeMap.navigate(): List<String> {
    var instruction = 0
    var node = "AAA"
    val route = mutableListOf<String>()

    while (node != "ZZZ") {
        route.add(node)
        node = when (instructions[instruction]) {
            'L' -> nodes[node]!!.first
            'R' -> nodes[node]!!.second
            else -> throw Exception("Unknown instruction: ${instructions[instruction]}")
        }
        instruction = (instruction + 1) % instructions.size
    }

    route.add(node)
    return route
}
