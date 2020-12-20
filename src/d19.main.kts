import java.io.File

sealed class Rule {
    abstract fun match(s: String): List<Pair<Boolean, Int>>
    data class Literal(val c: Char) : Rule() {
        override fun match(s: String) = if (s.firstOrNull() == c) {
            listOf(true to 1)
        } else {
            listOf(false to 0)
        }
    }

    data class Or(val sub: List<Rule>) : Rule() {
        override fun match(s: String) = sub
            .flatMap { it.match(s) }
            .filter { (matched, _) -> matched }
            .let { if (it.isEmpty()) listOf(false to 0) else it }
    }

    data class RuleRef(val ref: Int) : Rule() {
        override fun match(s: String) = rules[ref]!!.match(s)
    }

    data class RefSeq(val refs: List<RuleRef>) : Rule() {
        override fun match(s: String) =
            refs
                .fold(listOf(true to 0)) { acc, currRef ->
                    acc.flatMap { (matchSoFar, lengthSoFar) ->
                        if (matchSoFar) {
                            currRef
                                .match(s.drop(lengthSoFar))
                                .filter { (matched, _) -> matched }
                                .map { (matched, nextLen) ->
                                    matched to lengthSoFar + nextLen
                                }
                        } else {
                            listOf(matchSoFar to lengthSoFar)
                        }
                    }
                }
                .let { if (it.isEmpty()) listOf(false to 0) else it }
    }

    companion object {
        var rules: MutableMap<Int, Rule> = mutableMapOf()
    }
}

fun parseRule(text: String): Rule =
    when {
        text.contains('"') -> Rule.Literal(
            Regex(""""(\w)"""").find(text)!!.groupValues[1][0]
        )
        text.contains('|') -> Rule.Or(text.split('|').map(::parseRule))
        else -> {
            val parts = text.trim().split(" ")
            if (parts.size == 1) {
                Rule.RuleRef(parts[0].toInt())
            } else {
                Rule.RefSeq(parts.map { Rule.RuleRef(it.toInt()) })
            }
        }
    }

Rule.rules = File("./data/d19.rules.txt").readLines().map {
    val (num, ruleText) = it.split(":")
    num.toInt() to parseRule(ruleText)
}.toMap().toMutableMap()

val messages = File("./data/d19.messages.txt").readLines().map(String::trim)

fun completeMatch(rule: Rule, s: String): Boolean {
    val matches = rule.match(s)
    return matches.any { (matched, matchLen) -> matched && matchLen == s.length }
}

println("Part 1:")
println(messages.count { m -> completeMatch(Rule.rules[0]!!, m) })

println("Part 2:")
Rule.rules[8] =
    Rule.Or(
        listOf(
            Rule.RuleRef(42),
            Rule.RefSeq(listOf(Rule.RuleRef(42), Rule.RuleRef(8)))
        )
    )
Rule.rules[11] =
    Rule.Or(
        listOf(
            Rule.RefSeq(
                listOf(Rule.RuleRef(42), Rule.RuleRef(31))
            ),
            Rule.RefSeq(listOf(Rule.RuleRef(42), Rule.RuleRef(11), Rule.RuleRef(31)))
        )
    )
println(messages.count { m -> completeMatch(Rule.rules[0]!!, m) })
