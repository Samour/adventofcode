package com.adventofcode.samour.aoc2022.day5.parse

import com.adventofcode.samour.aoc2022.day5.CargoStructure
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test

internal class ParseCargoStructureTest {

    @Test
    fun `should parse sample structure correctly`() {
        val sourceLines = """
            [D]        
            [N] [C]    
            [Z] [M] [P]
        """.trimIndent().split("\n")
        val columns = " 1   2   3 "

        val result = parseCargoStructure(sourceLines, columns)

        assertThat(result).isEqualTo(
            CargoStructure(
                stacks = listOf(
                    listOf('D', 'N', 'Z'),
                    listOf('C', 'M'),
                    listOf('P'),
                )
            )
        )
    }

    @Test
    fun `should parse sample structure without trailing spaces correctly`() {
        val sourceLines = """
            [D]
            [N] [C]
            [Z] [M] [P]
        """.trimIndent().split("\n")
        val columns = " 1   2   3"

        val result = parseCargoStructure(sourceLines, columns)

        assertThat(result).isEqualTo(
            CargoStructure(
                stacks = listOf(
                    listOf('D', 'N', 'Z'),
                    listOf('C', 'M'),
                    listOf('P'),
                )
            )
        )
    }
}
