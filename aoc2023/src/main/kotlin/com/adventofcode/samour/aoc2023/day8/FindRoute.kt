package com.adventofcode.samour.aoc2023.day8

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun findRoute(fname: String, simulNavigate: Boolean) = readResource("day8/$fname").use { file ->
    file.parseNodeMap().navigate(simulNavigate)
}
