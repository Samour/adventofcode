package com.adventofcode.samour.aoc2022.day8

data class Tree(
    val identifier: String,
    val height: Int,
)

data class TreeMap(
    val trees: List<List<Tree>>,
)
