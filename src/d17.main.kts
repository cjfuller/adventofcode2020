import java.io.File
import kotlin.math.min
import kotlin.math.max


data class Coord(val x: Int, val y: Int, val z: Int, val w: Int) {
    fun isActiveNext3D(state: Set<Coord>): Boolean {
        val neighbors = sequence {
            (-1..1).map { xInc ->
                (-1..1).map { yInc ->
                    (-1..1).map { zInc ->
                        if (xInc == 0 && yInc == 0 && zInc == 0) {
                            // pass
                        } else {
                            yield(Coord(x + xInc, y + yInc, z + zInc, w))
                        }
                    }
                }
            }
        }.toList()
        val activeNeighborCount = neighbors.filter { it in state }.count()
        return ((this in state && activeNeighborCount == 2) || (activeNeighborCount == 3))
    }

    fun isActiveNext4D(state: Set<Coord>): Boolean {
        val neighbors = sequence {
            (-1..1).map { xInc ->
                (-1..1).map { yInc ->
                    (-1..1).map { zInc ->
                        (-1..1).map { wInc ->
                            if (xInc == 0 && yInc == 0 && zInc == 0 && wInc == 0) {
                                // pass
                            } else {
                                yield(Coord(x + xInc, y + yInc, z + zInc, w + wInc))
                            }
                        }
                    }
                }
            }
        }.toList()
        val activeNeighborCount = neighbors.filter { it in state }.count()
        return ((this in state && activeNeighborCount == 2) || (activeNeighborCount == 3))
    }
}

val initialState = File("./data/d17.txt")
    .readLines()
    .withIndex()
    .flatMap { (rIdx, r) ->
        r.withIndex().mapNotNull { (cIdx, c) ->
            if (c == '#') {
                Coord(cIdx, rIdx, 0, 0)
            } else {
                null
            }
        }
    }

fun nextState3D(currState: Set<Coord>): Set<Coord> {
    val mins = currState.fold(Coord(-1, -1, -1, 0)) { acc, curr ->
        val (ax, ay, az) = acc
        val (cx, cy, cz) = curr
        Coord(min(ax, cx - 1), min(ay, cy - 1), min(az, cz - 1), 0)
    }
    val maxes = currState.fold(Coord(1, 1, 1, 0)) { acc, curr ->
        val (ax, ay, az) = acc
        val (cx, cy, cz) = curr
        Coord(max(ax, cx + 1), max(ay, cy + 1), max(az, cz + 1), 0)
    }
    val (mnX, mnY, mnZ) = mins
    val (mxX, mxY, mxZ) = maxes
    val nextState: MutableSet<Coord> = mutableSetOf()
    (mnX..mxX).forEach { x ->
        (mnY..mxY).forEach { y ->
            (mnZ..mxZ).forEach { z ->
                val c = Coord(x, y, z, 0)
                if (c.isActiveNext3D(currState)) {
                    nextState.add(c)
                }
            }
        }
    }
    return nextState
}

fun nextState4D(currState: Set<Coord>): Set<Coord> {
    val mins = currState.fold(Coord(-1, -1, -1, -1)) { acc, curr ->
        val (ax, ay, az, aw) = acc
        val (cx, cy, cz, cw) = curr
        Coord(min(ax, cx - 1), min(ay, cy - 1), min(az, cz - 1), min(aw, cw - 1))
    }
    val maxes = currState.fold(Coord(1, 1, 1, 1)) { acc, curr ->
        val (ax, ay, az, aw) = acc
        val (cx, cy, cz, cw) = curr
        Coord(max(ax, cx + 1), max(ay, cy + 1), max(az, cz + 1), max(aw, cw + 1))
    }
    val (mnX, mnY, mnZ, mnW) = mins
    val (mxX, mxY, mxZ, mxW) = maxes
    val nextState: MutableSet<Coord> = mutableSetOf()
    (mnX..mxX).forEach { x ->
        (mnY..mxY).forEach { y ->
            (mnZ..mxZ).forEach { z ->
                (mnW..mxW).forEach { w ->
                    val c = Coord(x, y, z, w)
                    if (c.isActiveNext4D(currState)) {
                        nextState.add(c)
                    }
                }
            }
        }
    }
    return nextState
}

println("Part 1:")
var state = initialState.toSet()
repeat(6) { state = nextState3D(state) }
println(state.size)

println("Part 2:")
state = initialState.toSet()
repeat(6) { state = nextState4D(state) }
println(state.size)
