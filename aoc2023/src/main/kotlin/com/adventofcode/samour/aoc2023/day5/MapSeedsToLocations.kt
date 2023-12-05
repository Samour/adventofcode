package com.adventofcode.samour.aoc2023.day5

import com.adventofcode.samour.aoc2023.resources.ResourceReader.readResource
import java.math.BigInteger

fun mapSeedsToLocations(fname: String, collapseMaps: Boolean): Map<BigInteger, BigInteger> =
    readResource("day5/$fname").use { file ->
        val plantingDetails = file.parseAttributeMaps().let {
            if (collapseMaps) {
                it.copy(attributeMapChain = collapseMaps(it.attributeMapChain))
            } else {
                it
            }
        }

        plantingDetails.seeds.associateWith {
            plantingDetails.attributeMapChain.convertValue(
                AttributeType.SEED,
                it,
                AttributeType.LOCATION,
            )
        }
    }

fun findClosestLocationForSeedRanges(fname: String): BigInteger = readResource("day5/$fname")
    .use { file ->
        val plantingDetails = file.parseAttributeMaps().let {
            it.copy(attributeMapChain = collapseMaps(it.attributeMapChain))
        }

        plantingDetails.attributeMapChain
            .maps[AttributeType.SEED]!!
            .portions
            .sortedBy {
                it.srcStart + it.offset
            }.first { range ->
                plantingDetails.seedRanges.any { (seedLower, seedUpper) ->
                    range.srcStart < seedUpper && range.srcStart + range.rangeSize > seedLower
                }
            }.let { it.srcStart + it.offset }
    }
