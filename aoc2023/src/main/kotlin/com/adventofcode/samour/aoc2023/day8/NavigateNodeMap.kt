package com.adventofcode.samour.aoc2023.day8

import java.math.BigInteger

fun NodeMap.navigate(simulNavigate: Boolean): BigInteger {
    var instruction = 0
    var node = if (simulNavigate) {
        nodes.keys.filter { it.endsWith("A") }
    } else {
        listOf("AAA")
    }
    var steps = 0
    val partialSolution = mutableMapOf<Int, Int>()

    while (!node.isAtEnd(simulNavigate) && partialSolution.size < node.size) {
        steps++
        node = node.map {
            when (instructions[instruction]) {
                'L' -> nodes[it]!!.first
                'R' -> nodes[it]!!.second
                else -> throw Exception("Unknown instruction: ${instructions[instruction]}")
            }
        }
        if (simulNavigate) {
            node.forEachIndexed { i, name ->
                if (name.endsWith("Z") && !partialSolution.containsKey(i)) {
                    partialSolution[i] = steps
                }
            }
        }
        instruction = (instruction + 1) % instructions.size
    }

    if (node.isAtEnd(simulNavigate)) {
        return steps.toBigInteger()
    }

    return partialSolution.values.map { it.toBigInteger() }
        .reduce(::lcm)
}

private fun List<String>.isAtEnd(simulNavigate: Boolean): Boolean = if (simulNavigate) {
    all { it.endsWith("Z") }
} else {
    first() == "ZZZ"
}

private tailrec fun gcd(a: BigInteger, b: BigInteger): BigInteger {
    if (b > a) {
        return gcd(b, a)
    }

    if (b == BigInteger.ZERO) {
        return a
    }

    return gcd(b, a % b)
}

private fun lcm(a: BigInteger, b: BigInteger): BigInteger {
    return (a * b) / gcd(a, b)
}
