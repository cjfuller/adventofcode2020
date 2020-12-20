import java.io.File

data class Bus(val number: Long, val offset: Long) {
    fun arrivesAtCorrectOffset(t: Long): Boolean =
        (t + offset) % number == 0L
}

fun parseInput(): Pair<Long, List<Bus>> {
    val lines = File("./data/d13.txt").readLines()
    val departureTime = lines[0].toLong()
    val busses = lines[1]
        .split(",")
        .withIndex()
        .map { (i, v) ->
            Bus(
                number = if (v == "x") {
                    -1L
                } else {
                    v.toLong()
                },
                offset = i.toLong()
            )
        }
        .filter { (v, _) -> v != -1L }
    return Pair(departureTime, busses)
}

fun delayForBus(departureTime: Long, bus: Long): Long =
    bus - (departureTime % bus)

val (departureTime, bussesWithOffset) = parseInput()
val busses = bussesWithOffset.map { it.number }
val bestBus = busses.minByOrNull { delayForBus(departureTime, it) }!!

println("Part 1:")
println("Best bus: $bestBus")
println("Result: ${bestBus * delayForBus(departureTime, bestBus)}")

println("Part 2:")

val orderedBusses = bussesWithOffset.sortedBy { it.number }.reversed()

fun findTForPair(lhs: Bus, rhs: Bus): Long {
    var t: Long = (lhs.number - lhs.offset)
    while (!rhs.arrivesAtCorrectOffset(t)) {
        t += lhs.number
    }
    return t
}

val result = orderedBusses.reduce { b0, b1 ->
    val t = findTForPair(b0, b1)
    val combinedNum = b0.number * b1.number
    println(t)
    Bus(number = combinedNum, offset = combinedNum - (t % combinedNum))
}
println(result)
