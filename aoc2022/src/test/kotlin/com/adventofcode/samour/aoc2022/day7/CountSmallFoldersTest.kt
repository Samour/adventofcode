package com.adventofcode.samour.aoc2022.day7

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class CountSmallFoldersTest {

    @Test
    fun `should correctly count folder sizes from sample data`() {
        val result = countSmallFolders("sample.txt")

        assertThat(result).isEqualTo(95437)
    }

    @Test
    fun `should correctly count folder sizes from problem data`() {
        val result = countSmallFolders("data.txt")

        assertThat(result).isEqualTo(1297159)
    }
}
