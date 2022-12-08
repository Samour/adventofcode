package com.adventofcode.samour.aoc2022.day8

import com.adventofcode.samour.aoc2022.resources.ResourceReader.readResource

fun parseTreeMap(fname: String): TreeMap {
    return TreeMap(
        readResource("day8/$fname").lineSequence()
            .mapIndexed { i, row ->
                row.toCharArray().mapIndexed { j, height ->
                    Tree(
                        identifier = "($i,$j)",
                        height = "$height".toInt(),
                    )
                }
            }.toList()
    )
}
