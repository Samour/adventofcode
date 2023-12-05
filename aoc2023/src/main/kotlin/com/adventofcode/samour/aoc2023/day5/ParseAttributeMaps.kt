package com.adventofcode.samour.aoc2023.day5

import java.io.BufferedReader
import java.math.BigInteger

data class PlantingDetails(
    val seeds: Set<BigInteger>,
    val attributeMapChain: AttributeMapChain,
)

private val attributePattern = AttributeType.entries
    .joinToString("|") { it.name.lowercase() }
private val mapdefRegex = Regex("($attributePattern)-to-($attributePattern) map:")

fun BufferedReader.parseAttributeMaps(): PlantingDetails {
    lateinit var seeds: Set<BigInteger>
    val attributeMaps = mutableMapOf<AttributeType, AttributeMap>()

    lateinit var srcType: AttributeType
    lateinit var destType: AttributeType
    var mapPortions = mutableListOf<AttributeMapPortion>()

    readLines().forEach { line ->
        if (line.startsWith("seeds: ")) {
            seeds = line.substring(7).split(" ")
                .map { it.trim().toBigInteger() }
                .toSet()
            return@forEach
        }

        val mapTypeMatch = mapdefRegex.matchEntire(line)
        if (mapTypeMatch != null) {
            if (mapPortions.isNotEmpty()) {
                attributeMaps[srcType] = AttributeMap(
                    destType = destType,
                    portions = mapPortions,
                )
                mapPortions = mutableListOf()
            }
            srcType = AttributeType.valueOf(mapTypeMatch.groups[1]!!.value.uppercase())
            destType = AttributeType.valueOf(mapTypeMatch.groups[2]!!.value.uppercase())
        } else if (line.isNotBlank()) {
            val (destStart, srcStart, rangeSize) = line.split(" ").map { it.trim().toBigInteger() }
            mapPortions.add(
                AttributeMapPortion(
                    srcStart = srcStart,
                    destStart = destStart,
                    rangeSize = rangeSize,
                ),
            )
        }
    }

    if (mapPortions.isNotEmpty()) {
        attributeMaps[srcType] = AttributeMap(
            destType = destType,
            portions = mapPortions,
        )
        mapPortions = mutableListOf()
    }

    return PlantingDetails(
        seeds = seeds,
        attributeMapChain = AttributeMapChain(attributeMaps),
    )
}
