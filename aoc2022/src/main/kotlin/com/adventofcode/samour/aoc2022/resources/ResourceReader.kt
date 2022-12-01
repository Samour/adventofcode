package com.adventofcode.samour.aoc2022.resources

import java.io.BufferedReader

object ResourceReader {

    fun readResource(fname: String): BufferedReader =
        javaClass.classLoader.getResourceAsStream(fname).bufferedReader()
}
