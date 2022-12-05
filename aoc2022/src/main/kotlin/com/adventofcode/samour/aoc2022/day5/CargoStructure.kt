package com.adventofcode.samour.aoc2022.day5

data class CargoStructure(
    /**
     * Outer list = column number
     *
     * Inner list = height in stack
     * Index 0 = top of stack
     * Index N = bottom of stack
     */
    val stacks: List<List<Char>>,
)
