package com.adventofcode.samour.aoc2023.day5

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource
import java.math.BigInteger

fun loadMaps(fname: String) = readResource("day5/$fname").use { file ->
    file.parseAttributeMaps()
}

fun mapSeedsToLocations(plantingDetails: PlantingDetails): Map<BigInteger, BigInteger> {
    return plantingDetails.seeds.associateWith {
        plantingDetails.attributeMapChain.convertValue(
            AttributeType.SEED,
            it,
            AttributeType.LOCATION,
        )
    }
}
