package com.adventofcode.samour.aoc2023.day5

import java.io.BufferedReader
import java.math.BigInteger

data class PlantingDetails(
    val seeds: Set<BigInteger>,
    val seedRanges: List<Pair<BigInteger, BigInteger>>,
    val attributeMapChain: AttributeMapChain,
)

private val attributePattern = AttributeType.entries
    .joinToString("|") { it.name.lowercase() }
private val mapdefRegex = Regex("($attributePattern)-to-($attributePattern) map:")

fun BufferedReader.parseAttributeMaps(): PlantingDetails {
    lateinit var seeds: Set<BigInteger>
    lateinit var seedRanges: List<Pair<BigInteger, BigInteger>>
    val attributeMaps = mutableMapOf<AttributeType, AttributeMap>()

    lateinit var srcType: AttributeType
    lateinit var destType: AttributeType
    var mapPortions = mutableListOf<AttributeMapPortion>()

    readLines().forEach { line ->
        if (line.startsWith("seeds: ")) {
            val ordered = line.substring(7).split(" ")
                .map { it.trim().toBigInteger() }
            seedRanges = ordered.chunked(2) { (a, b) -> a to a + b }
            seeds = ordered.toSet()
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
                    offset = destStart - srcStart,
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
        seedRanges = seedRanges,
        attributeMapChain = AttributeMapChain(attributeMaps),
    )
}
