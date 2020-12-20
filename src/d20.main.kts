import java.io.File

fun Long.mirror(): Long = (0 until 10).map { i ->
    if (this and (1L shl i) != 0L) {
        1L shl (9 - i)
    } else {
        0
    }
}.sum()

data class Tile(val number: Long, val borders: List<Long>) {
    fun mirrorLR(): Tile =
        this.copy(
            borders = this.borders.take(2).map { it.mirror() } + this.borders.drop(2))

    fun mirrorUD(): Tile =
        this.copy(
            borders = this.borders.take(2) + this.borders.drop(2).map { it.mirror() })

    fun rot90(): Tile =
        this.copy(
            borders = listOf(
                this.borders[3],
                this.borders[0].mirror(),
                this.borders[1],
                this.borders[2].mirror()
            )
        )

    fun allRotations() = listOf(
        this,
        this.rot90(),
        this.rot90().rot90(),
        this.rot90().rot90().rot90()
    )

    fun allOrientations() = allRotations().flatMap { r ->
        listOf(
            r,
            r.mirrorLR(),
            r.mirrorUD()
        )
    }


    fun allBorders(): Set<Long> = allOrientations().flatMap { it.borders }.toSet()

    fun hasTwoUnmatched(unmatchedBorders: Set<Long>): Boolean =
        borders.count { it in unmatchedBorders } >= 2
}

fun numericValueForSeq(s: Iterable<Char>): Long =
    s.withIndex().fold(0) { acc, (i, c) ->
        if (c == '#') {
            acc + (1 shl i)
        } else {
            acc
        }
    }

val tiles = File("./data/d20.txt")
    .readLines()
    .chunked(12) { chunk ->
        val numLine = chunk.take(1)[0]
        val grid = chunk.drop(1).take(10)
        Tile(
            Regex("""Tile (\d+):""").find(numLine)!!.groupValues[1].toLong(),
            listOf(
                numericValueForSeq(grid.first().asIterable()),
                numericValueForSeq(grid.last().asIterable()),
                numericValueForSeq(grid.map { it.first() }),
                numericValueForSeq(grid.map { it.last() })
            )
        )
    }

val tileLookup: MutableMap<Long, List<Tile>> = mutableMapOf()

tiles.forEach { tile ->
    tile.allBorders().forEach { border ->
        tileLookup[border] = (tileLookup[border] ?: listOf()) + tile
    }
}

val unmatchedNumbers = tileLookup.filter { (_, v) -> v.size == 1 }.keys.toSet()

// Conceivable that this might not work if the corners actually do
// match another tile but there's no layout that satisfies all the
// other constraints.
// However, this works in practice.
val corners = tiles.filter { t ->
    t.allOrientations().all { o -> o.hasTwoUnmatched(unmatchedNumbers) }
}

println("Part 1:")
println(corners.map { it.number }.reduce { a, b -> a * b })

