import java.io.File

enum class Op {
    PLUS, TIMES
}

sealed class Token {
    data class Number(val value: Long) : Token() {
        override fun toString(): String = "n:$value"
    }

    data class Operator(val op: Op) : Token() {
        override fun toString(): String = "o:$op"
    }

    object LParen : Token() {
        override fun toString(): String = "p:("
    }

    object RParen : Token() {
        override fun toString(): String = "p:)"
    }
}

sealed class Expression {
    abstract fun evaluate(): Long

    data class Operation(val op: Op, val lhs: Expression, val rhs: Expression) :
        Expression() {
        override fun evaluate(): Long =
            when (op) {
                Op.PLUS -> lhs.evaluate() + rhs.evaluate()
                Op.TIMES -> lhs.evaluate() * rhs.evaluate()
            }

        override fun toString(): String = "($op $lhs $rhs)"
    }

    data class Number(val value: Long) : Expression() {
        override fun evaluate(): Long = value
        override fun toString(): String = value.toString()
    }
}


fun tokenize(s: String): List<Token> {
    if (s.isEmpty()) {
        return listOf()
    }
    return when (s.first()) {
        '(' -> listOf(Token.LParen) + tokenize(s.drop(1))
        ')' -> listOf(Token.RParen) + tokenize(s.drop(1))
        '+' -> listOf(Token.Operator(Op.PLUS)) + tokenize(s.drop(1))
        '*' -> listOf(Token.Operator(Op.TIMES)) + tokenize(s.drop(1))
        ' ' -> tokenize(s.drop(1))
        else -> {
            val (_, num, rest) = Regex("""^(\d+)(.*)$""").find(s)!!.groupValues
            listOf(Token.Number(num.toLong())) + tokenize(rest)
        }
    }
}

sealed class ParseResult {
    object ExprAddedToStack : ParseResult()
    object CloseParen : ParseResult()
}

fun parseNext(
    tokens: MutableList<Token>,
    stack: MutableList<Expression>,
    usePrecedence: Boolean = true,
): ParseResult {
    when (val x = tokens.removeFirst()) {
        is Token.Number -> {
            stack.add(Expression.Number(x.value))
            return ParseResult.ExprAddedToStack
        }
        is Token.Operator -> {
            if (usePrecedence) {
                when (x.op) {
                    Op.PLUS -> parseNext(tokens, stack, usePrecedence)
                    Op.TIMES -> {
                        var result: ParseResult = ParseResult.ExprAddedToStack
                        while (result != ParseResult.CloseParen && tokens.isNotEmpty()) {
                            result = parseNext(tokens, stack, usePrecedence)
                        }
                        if (result == ParseResult.CloseParen) {
                            tokens.add(0, Token.RParen)
                        }
                    }
                }
            } else {
                parseNext(tokens, stack, usePrecedence)
            }
            val rhs = stack.removeLast()
            val lhs = stack.removeLast()
            stack.add(
                Expression.Operation(
                    x.op,
                    lhs,
                    rhs
                )
            )
            return ParseResult.ExprAddedToStack
        }
        is Token.LParen -> {
            val newStack = mutableListOf<Expression>()
            var result: ParseResult = ParseResult.ExprAddedToStack
            while (result != ParseResult.CloseParen && tokens.isNotEmpty()) {
                result = parseNext(tokens, newStack, usePrecedence)
            }
            if (newStack.size != 1) {
                throw Exception("Unexpected stack state: $newStack")
            }
            stack.addAll(newStack)
            return ParseResult.ExprAddedToStack
        }
        is Token.RParen -> {
            return ParseResult.CloseParen
        }
    }
}

fun _parse(
    tokens: MutableList<Token>,
    stack: MutableList<Expression>,
    usePrecedence: Boolean = true
) {
    while (tokens.isNotEmpty()) {
        parseNext(tokens, stack, usePrecedence)
    }
}


fun parse(tokens: MutableList<Token>, usePrecedence: Boolean = true): Expression {
    val stack = mutableListOf<Expression>()
    _parse(tokens, stack, usePrecedence)
    if (stack.size != 1) {
        throw Exception("Unexpected stack state: $stack")
    }
    return stack.removeFirst()
}

val expressions = File("./data/d18.txt")
    .readLines()
    .map(::tokenize)
    .map { parse(it.toMutableList(), usePrecedence = false) }

println("Part 1:")
println(expressions.map { it.evaluate() }.sum())

println("Part 2:")

val tests = mapOf(
    "1 + (2 * 3) + (4 * (5 + 6))" to 51,
    "2 * 3 + (4 * 5)" to 46,
    "5 + (8 * 3 + 9 + 3 * 4 * 3)" to 1445,
    "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" to 669060,
    "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" to 23340
)

// for ((test, expected) in tests) {
//     println(test)
//     val ast = parse(
//         tokenize(test).toMutableList(),
//         usePrecedence = true
//     )
//     val result = ast.evaluate()
//     println(ast)
//     println("expected $expected, got $result")
// }
val p2Expressions = File("./data/d18.txt")
    .readLines()
    .map(::tokenize)
    .map { parse(it.toMutableList(), usePrecedence = true) }

println(p2Expressions.map { it.evaluate() }.sum())
