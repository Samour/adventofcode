package com.adventofcode.samour.aoc2022.day6

private data class PacketWindow(
    val window: List<Char>,
    val charCount: Map<Char, Int>,
    val windowSize: Int,
) {

    fun pushChar(char: Char): PacketWindow {
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

        return PacketWindow(
            window = window,
            charCount = charCount.toMap(),
            windowSize = windowSize,
        )
    }

    companion object {
        fun createWindow(windowSize: Int) = PacketWindow(
            window = emptyList(),
            charCount = emptyMap(),
            windowSize = windowSize,
        )
    }
}

fun findPacketStart(data: String): Int {
    data.foldIndexed(PacketWindow.createWindow(4)) { i, window, c ->
        window.pushChar(c).also {
            if (it.charCount.size == it.windowSize) {
                return i + 1
            }
        }
    }

    throw IllegalArgumentException("Could not find a sequence of 4 unique characters")
}
