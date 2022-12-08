package com.adventofcode.samour.aoc2022.day8

fun countVisibleTrees(fname: String): Int {
    return generateAllTreeLines(parseTreeMap(fname))
        .flatMap { visibleTrees(it) }
        .map { it.identifier }
        .toSet()
        .size
}

private fun generateAllTreeLines(trees: TreeMap): List<List<Tree>> {
    var treeLines = trees.trees + trees.trees.map { it.reversed() }
    for (i in (0..trees.trees.lastIndex)) {
        val transposedLine = mutableListOf<Tree>()
        for (j in (0..trees.trees.first().lastIndex)) {
            transposedLine.add(trees.trees[j][i])
        }
        treeLines += listOf(
            transposedLine,
            transposedLine.reversed(),
        )
    }

    return treeLines
}

private fun visibleTrees(trees: List<Tree>): List<Tree> {
    val visibleTrees = mutableListOf<Tree>()
    var highestToPoint = -1
    trees.forEach {
        if (it.height > highestToPoint) {
            visibleTrees.add(it)
            highestToPoint = it.height
        }
    }

    return visibleTrees
}
