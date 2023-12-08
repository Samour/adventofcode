package com.adventofcode.samour.aoc2023.day8

data class NodeMap(
    val instructions: List<Char>,
    val nodes: Map<String, Pair<String, String>>,
)
