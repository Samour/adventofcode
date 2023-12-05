package com.adventofcode.samour.aoc2023.day5

import java.math.BigInteger

data class AttributeMapChain(
    val maps: Map<AttributeType, AttributeMap>,
) {

    fun convertValue(srcType: AttributeType, source: BigInteger, destType: AttributeType): BigInteger {
        if (srcType == destType) {
            return source
        }

        return maps[srcType]?.let {
            convertValue(it.destType, it.convertValue(source), destType)
        } ?: source
    }
}

data class AttributeMap(
    val destType: AttributeType,
    val portions: List<AttributeMapPortion>,
) {

    fun convertValue(source: BigInteger): BigInteger =
        portions
            .firstNotNullOfOrNull { it.convertValue(source) }
            ?: source
}

data class AttributeMapPortion(
    val srcStart: BigInteger,
    val offset: BigInteger,
    val rangeSize: BigInteger,
) {

    override fun toString(): String = "srcRange = [$srcStart, ${srcStart + rangeSize})"

    fun convertValue(source: BigInteger): BigInteger? = (source + offset).takeIf {
        source >= srcStart && source < srcStart + rangeSize
    }

    fun split(firstRangeSize: BigInteger): Pair<AttributeMapPortion, AttributeMapPortion> {
        assert(firstRangeSize > BigInteger.ZERO)
        assert(firstRangeSize < rangeSize)
        val first = copy(rangeSize = firstRangeSize)
        val second = copy(
            srcStart = srcStart + firstRangeSize,
            rangeSize = rangeSize - firstRangeSize,
        )

        return first to second
    }

    fun combine(downstreamMap: AttributeMapPortion): List<AttributeMapPortion> {
        val combination = mutableListOf<AttributeMapPortion>()
        if ()
    }
}

enum class AttributeType {
    SEED,
    SOIL,
    FERTILIZER,
    WATER,
    LIGHT,
    TEMPERATURE,
    HUMIDITY,
    LOCATION,
}
