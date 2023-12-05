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

    while (sourceRanges.isNotEmpty()) {
        val srcStart = sourceRanges.first().let { it.srcStart + it.offset }
        val srcEnd = srcStart + sourceRanges.first().rangeSize

        val overlappingDest = destRanges.firstOrNull {
            srcStart < it.srcStart + it.rangeSize && srcEnd > it.srcStart
        }
        if (overlappingDest == null) {
            finalRanges.add(sourceRanges.removeFirst())
            continue
        }

        val destStart = overlappingDest.srcStart
        val destEnd = destStart + overlappingDest.rangeSize

        /**
         * src:         |----------|
         * dest:                |-------|
         *
         * src:         |----------|
         * dest:            |---|
         */
        if (srcStart < destStart) {
            val (portionToAdd, portionToCalculate) = sourceRanges.removeFirst()
                .split(destStart - srcStart)
            finalRanges.add(portionToAdd)
            sourceRanges.add(0, portionToCalculate)

            continue
        }

        /**
         * src:              |----------|
         * dest:        |-------|
         *
         * src:         |----|
         * dest:     |----------|
         */
        if (destStart < srcStart) {
            val (portionToAdd, portionToCalculate) = destRanges.removeFirst()
                .split(srcStart - destStart)
            finalRanges.add(portionToAdd)
            sourceRanges.add(0, portionToCalculate)

            continue
        }

        /**
         * src:      |----------|
         * dest:     |-------|
         */
        if (destEnd < srcEnd) {
            val (srcPortionToAdd, portionToCalculate) = sourceRanges.removeFirst()
                .split(destEnd - srcStart)
            val destPortionToAdd = destRanges.removeFirst()
            assert(srcPortionToAdd.rangeSize == destPortionToAdd.rangeSize)
            AttributeMapPortion(
                srcStart = srcPortionToAdd.srcStart,
                offset = srcPortionToAdd.offset + destPortionToAdd.offset,
                rangeSize = srcPortionToAdd.rangeSize,
            ).takeIf { it.offset != BigInteger.ZERO }
                ?.let { finalRanges.add(it) }
            sourceRanges.add(0, portionToCalculate)

            continue
        }

        /**
         * src:      |----|
         * dest:     |----------|
         */
        if (srcEnd < destEnd) {
            val srcPortionToAdd = sourceRanges.removeFirst()
            val (destPortionToAdd, portionToCalculate) = destRanges.removeFirst()
                .split(srcPortionToAdd.rangeSize)
            assert(srcPortionToAdd.rangeSize == destPortionToAdd.rangeSize)
            AttributeMapPortion(
                srcStart = srcPortionToAdd.srcStart,
                offset = srcPortionToAdd.offset + destPortionToAdd.offset,
                rangeSize = srcPortionToAdd.rangeSize,
            ).takeIf { it.offset != BigInteger.ZERO }
                ?.let { finalRanges.add(it) }
            destRanges.add(0, portionToCalculate)

            continue
        }

        /**
         * src:      |-------|
         * dest:     |-------|
         */
        val srcPortionToAdd = sourceRanges.removeFirst()
        val destPortionToAdd = destRanges.removeFirst()
        assert(srcPortionToAdd.rangeSize == destPortionToAdd.rangeSize)
        AttributeMapPortion(
            srcStart = srcPortionToAdd.srcStart,
            offset = srcPortionToAdd.offset + destPortionToAdd.offset,
            rangeSize = srcPortionToAdd.rangeSize,
        ).takeIf { it.offset != BigInteger.ZERO }
            ?.let { finalRanges.add(it) }
    }

    finalRanges.addAll(destRanges)

    return AttributeMap(
        destType = destMap.destType,
        portions = finalRanges,
    )
}
