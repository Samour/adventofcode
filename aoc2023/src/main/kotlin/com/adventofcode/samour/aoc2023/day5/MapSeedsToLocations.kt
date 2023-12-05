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

fun collapseMaps(maps: AttributeMapChain): AttributeMapChain {
    val finalMaps = maps.maps.toMutableMap()
    while (finalMaps[AttributeType.SEED]!!.destType != AttributeType.LOCATION) {
        val seedMap = finalMaps[AttributeType.SEED]!!
        finalMaps[AttributeType.SEED] = combineMaps(seedMap, finalMaps[seedMap.destType]!!)
    }

    return AttributeMapChain(finalMaps)
}

private fun combineMaps(sourceMap: AttributeMap, destMap: AttributeMap): AttributeMap {
    val sourceRanges = sourceMap.portions.toMutableList()
    sourceRanges.sortBy { it.srcStart }
    val destRanges = destMap.portions.toMutableList()
    destRanges.sortBy { it.srcStart }
    val finalRanges = mutableListOf<AttributeMapPortion>()

    sourceRanges.forEach { srcRange ->
        val srcOffset = srcRange.offset
        var srcStart = srcRange.let { it.srcStart + it.offset }
        val srcEnd = srcStart + srcRange.rangeSize

        destRanges.filter {
            srcStart < it.srcStart + it.rangeSize && srcEnd > it.srcStart
        }.forEach { downstream ->
            if (srcStart < downstream.srcStart) {
                finalRanges.add(
                    AttributeMapPortion(
                        srcStart = srcStart - srcOffset,
                        offset = srcOffset,
                        rangeSize = downstream.srcStart - srcStart,
                    ),
                )
                srcStart = downstream.srcStart
            }
            AttributeMapPortion(
                srcStart = srcStart - srcOffset,
                offset = srcOffset + downstream.offset,
                rangeSize = listOf(
                    srcEnd - srcStart,
                    downstream.srcStart + downstream.rangeSize - srcStart,
                ).min(),
            ).also {
                srcStart += it.rangeSize
            }.takeIf { it.offset != BigInteger.ZERO }
                ?.let { finalRanges.add(it) }
        }

        if (srcStart < srcEnd) {
            finalRanges.add(
                AttributeMapPortion(
                    srcStart = srcStart - srcOffset,
                    offset = srcOffset,
                    rangeSize = srcEnd - srcStart,
                ),
            )
        }
    }

    destRanges.forEach { dest ->
        var destStart = dest.srcStart
        val destEnd = destStart + dest.rangeSize
        var lastEnd = BigInteger.ZERO
        sourceRanges.map { it.srcStart to it.srcStart + it.rangeSize }
            .forEach { (srcStart, srcEnd) ->
                if (destStart >= destEnd || destStart > srcEnd) {
                    // Pass
                } else if (destEnd < srcStart) {
                    finalRanges.add(
                        dest.copy(
                            srcStart = destStart,
                            rangeSize = destEnd - destStart,
                        ),
                    )
                    destStart = destEnd
                } else if (destStart < srcStart) {
                    finalRanges.add(
                        dest.copy(
                            srcStart = destStart,
                            rangeSize = listOf(
                                srcStart - destStart,
                                destEnd - destStart,
                            ).min(),
                        ),
                    )
                }
                if (destStart < srcEnd) {
                    destStart = srcEnd
                }
                lastEnd = srcEnd
            }

        if (destEnd > destStart && destEnd > lastEnd) {
            if (destStart > lastEnd) {
                finalRanges.add(
                    dest.copy(
                        srcStart = destStart,
                        rangeSize = destEnd - destStart,
                    ),
                )
            } else {
                finalRanges.add(
                    dest.copy(
                        srcStart = lastEnd,
                        rangeSize = destEnd - lastEnd,
                    ),
                )
            }
        }
    }

    return AttributeMap(
        destType = destMap.destType,
        portions = finalRanges,
    )
}
