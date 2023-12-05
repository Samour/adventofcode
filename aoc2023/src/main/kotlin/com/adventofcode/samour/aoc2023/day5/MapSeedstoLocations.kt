package com.adventofcode.samour.aoc2023.day5

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource

fun mapSeedsToLocations(fname: String) = readResource("day5/$fname").use { file ->
    val plantingDetails = file.parseAttributeMaps()
    plantingDetails.seeds.associateWith {
        plantingDetails.attributeMapChain.convertValue(
            AttributeType.SEED,
            it,
            AttributeType.LOCATION,
        )
    }
}
