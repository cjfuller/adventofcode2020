data class Stats(
    var mostRecentTurn: Int,
    var previousTurn: Int?
)

data class State(
    val spoken: MutableMap<Int, Stats>,
    var lastSpoken: Int,
    var index: Int
) {
    fun advance() {
        val (mostRecent, previous) = spoken[lastSpoken]!!
        val nextNum = if (previous == null) {
            0
        } else {
            mostRecent - previous
        }
        if (nextNum !in spoken) {
            spoken[nextNum] = Stats(index + 1, null)
        } else {
            spoken[nextNum]!!.previousTurn = spoken[nextNum]!!.mostRecentTurn
            spoken[nextNum]!!.mostRecentTurn = index + 1
        }
        this.lastSpoken = nextNum
        this.index += 1
    }
}

val input = listOf(1, 20, 8, 12, 0, 14)

fun initialState(): State = State(
    spoken = input
        .withIndex()
        .map { (idx, inputVal) -> inputVal to Stats(idx + 1, null) }
        .toMap().toMutableMap(),
    lastSpoken = input.last(),
    index = input.size
)

println("Part 1:")
var state = initialState()
while (state.index < 2020) {
    state.advance()
}
println(state.lastSpoken)

println("Part 2:")
state = initialState()
while (state.index < 30000000) {
    state.advance()
}

println(state.lastSpoken)
