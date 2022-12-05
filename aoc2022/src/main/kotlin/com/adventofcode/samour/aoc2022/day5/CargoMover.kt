package com.adventofcode.samour.aoc2022.day5

typealias CargoMover = (CargoStructure, MoveInstruction) -> CargoStructure

fun moveOneAtATime(cargoStructure: CargoStructure, moveInstruction: MoveInstruction): CargoStructure {
    val singleMoveInstruction = MoveInstruction(
        source = moveInstruction.source,
        destination = moveInstruction.destination,
        quantity = 1,
    )
    return (1..moveInstruction.quantity).fold(cargoStructure) { increment, _ ->
        moveMultipleAtOnce(increment, singleMoveInstruction)
    }
}

fun moveMultipleAtOnce(cargoStructure: CargoStructure, moveInstruction: MoveInstruction): CargoStructure =
    CargoStructure(
        stacks = cargoStructure.stacks.mapIndexed { i, containers ->
            when (i) {
                moveInstruction.source -> containers.subList(moveInstruction.quantity, containers.size)
                moveInstruction.destination -> cargoStructure.stacks[moveInstruction.source]
                    .subList(0, moveInstruction.quantity) + containers
                else -> containers
            }
        }
    )
