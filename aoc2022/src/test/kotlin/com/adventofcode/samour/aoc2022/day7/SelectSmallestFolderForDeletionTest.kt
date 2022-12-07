package com.adventofcode.samour.aoc2022.day7

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class SelectSmallestFolderForDeletionTest {

    @Test
    fun `should select smallest viable folder from sample file`() {
        val result = selectSmallestFolderForDeletion("sample.txt")

        assertThat(result).isEqualTo(24933642)
    }

    @Test
    fun `should select smallest viable folder from problem file`() {
        val result = selectSmallestFolderForDeletion("data.txt")

        assertThat(result).isEqualTo(3866390)
    }
}
