import java.io.File

data class Mask(val maskStr: String) {
    val orForm: Long = maskStr.replace(Regex("X"), "0").toLong(2)
    val andForm: Long = maskStr.replace(Regex("X"), "1").toLong(2)

    operator fun invoke(toValue: Long): Long {
        return (toValue and andForm) or orForm
    }

    fun addresses(baseAddr: Int): Sequence<Long> = sequence {
        val preFloatAddr = baseAddr.toLong() or orForm
        val floatingPositions =
            maskStr.reversed().withIndex().mapNotNull { (i, c) ->
                if (c == 'X') {
                    i
                } else {
                    null
                }
            }

        (0 until (1 shl floatingPositions.size)).forEach { floatingMask ->
            var building = preFloatAddr
            floatingPositions.withIndex().forEach { (i, pos) ->
                building = if (floatingMask and (1 shl i) == 0) {
                    building.setBitAt(pos, false)
                } else {
                    building.setBitAt(pos, true)
                }
            }
            yield(building)
        }

    }
}

fun parseMask(inputLine: String): Mask {
    return Mask(inputLine.substringAfter("mask = "))
}

data class Write(val address: Int, val value: Long) {
    companion object {
        val parser = Regex("""mem\[(\d+)\] = (\d+)""")
        fun parse(inputLine: String): Write {
            val (_, addr, value) = parser.find(inputLine)?.groupValues!!
            return Write(addr.toInt(), value.toLong())
        }
    }
}

data class Operation(val mask: Mask, val writes: List<Write>)

fun Long.setBitAt(target: Int, value: Boolean): Long {
    return if (value) {
        this or (1L shl target)
    } else {
        this and (1L shl target).inv()
    }
}

fun parseInput(): List<Operation> =
    File("./data/d14.txt").readLines()
        .fold(listOf()) { ops, currLine ->
            if (currLine.startsWith("mask = ")) {
                ops + Operation(parseMask(currLine), listOf())
            } else {
                ops.dropLast(1) + ops.last()
                    .let { it.copy(writes = it.writes + Write.parse(currLine)) }
            }
        }

println("Part 1:")

val memory: MutableMap<Int, Long> = mutableMapOf()
parseInput().forEach { op ->
    op.writes.forEach { w ->
        memory[w.address] = op.mask(w.value)
    }
}
println(memory.values.sum())

println("Part 2:")
val memoryP2: MutableMap<Long, Long> = mutableMapOf()
parseInput().forEach { op ->
    op.writes.forEach { w ->
        op.mask.addresses(w.address)
            .forEach { a -> memoryP2[a] = w.value }
    }
}
println(memoryP2.values.sum())