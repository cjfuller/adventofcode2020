import java.io.File

fun parseInput(): Pair<Int, List<Pair<Int, Int>>> {
    val lines = File("./data/d13.txt").readLines()
    val departureTime = Integer.parseInt(lines[0])
    val busses = lines[1]
        .split(",")
        .withIndex()
        .map { (i, v) ->
            Pair(
                if (v == "x") {
                    -1
                } else {
                    Integer.parseInt(v)
                }, i
            )
        }
        .filter { (v, _) -> v != -1 }
    return Pair(departureTime, busses)
}

fun delayForBus(departureTime: Int, bus: Int): Int =
    bus - (departureTime % bus)

val (departureTime, bussesWithOffset) = parseInput()
val busses = bussesWithOffset.map { it.first }
val bestBus = busses.minByOrNull { delayForBus(departureTime, it) }!!

println("Part 1:")
println("Best bus: $bestBus")
println("Result: ${bestBus * delayForBus(departureTime, bestBus)}")

println("Part 2:")
println(bussesWithOffset)