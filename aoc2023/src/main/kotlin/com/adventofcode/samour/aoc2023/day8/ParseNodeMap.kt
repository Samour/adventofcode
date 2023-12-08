package com.adventofcode.samour.aoc2023.day8

import java.io.BufferedReader

fun BufferedReader.parseNodeMap(): NodeMap {
    val instructions = readLine().trim().toCharArray().toList()
    val nodes = readLines().mapNotNull { parseNodeConnections(it) }
        .toMap()

    return NodeMap(
        instructions = instructions,
        nodes = nodes,
    )
}

private val nodeConnectionsPattern = "([0-9a-zA-Z]+)\\s+=\\s+\\(([0-9a-zA-Z]+),\\s+([0-9a-zA-Z]+)\\)".toRegex()

private fun parseNodeConnections(line: String): Pair<String, Pair<String, String>>? {
    val match = nodeConnectionsPattern.matchEntire(line.trim()) ?: return null

    return match.groups[1]!!.value to (match.groups[2]!!.value to match.groups[3]!!.value)
}
