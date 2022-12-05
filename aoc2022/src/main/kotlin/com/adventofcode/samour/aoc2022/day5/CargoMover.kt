package com.adventofcode.samour.aoc2022.day5

typealias CargoMover = (CargoStructure, MoveInstruction) -> CargoStructure

fun moveOneAtATime(cargoStructure: CargoStructure, moveInstruction: MoveInstruction): CargoStructure =
    (1..moveInstruction.quantity).fold(cargoStructure) { increment, _ ->
        increment.applySingleMove(moveInstruction)
    }

private fun CargoStructure.applySingleMove(moveInstruction: MoveInstruction): CargoStructure =
    CargoStructure(
        stacks = stacks.mapIndexed { i, containers ->
            if (i == moveInstruction.source) {
                containers.subList(1, containers.size)
            } else if (i == moveInstruction.destination) {
                listOf(stacks[moveInstruction.source][0]) + containers
            } else {
                containers
            }
        }
    )
