package com.adventofcode.samour.aoc2022.day6

private data class CharWindow(
    val window: List<Char>,
    val charCount: Map<Char, Int>,
    val windowSize: Int,
) {

    fun advanceWindow(char: Char): CharWindow {
        var window = this.window + listOf(char)
        val charCount = this.charCount.toMutableMap()
        charCount[char] = (charCount[char] ?: 0) + 1
        if (window.size > windowSize) {
            if (charCount[window[0]] == 1) {
                charCount.remove(window[0])
            } else {
                charCount[window[0]] = charCount[window[0]]!! - 1
            }
            window = window.subList(1, window.size)
        }

        return CharWindow(
            window = window,
            charCount = charCount.toMap(),
            windowSize = windowSize,
        )
    }

    companion object {
        fun createWindow(windowSize: Int) = CharWindow(
            window = emptyList(),
            charCount = emptyMap(),
            windowSize = windowSize,
        )
    }
}

fun findUniqueWindowStart(data: String, windowSize: Int): Int {
    data.foldIndexed(CharWindow.createWindow(windowSize)) { i, window, c ->
        window.advanceWindow(c).also {
            if (it.charCount.size == it.windowSize) {
                return i + 1
            }
        }
    }

    throw IllegalArgumentException("Could not find a sequence of $windowSize unique characters")
}
