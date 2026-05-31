package ch02

/**
 * 후위 표기식 계산기 (Calculator)
 * Rust/ch02/src/calculator.rs → Kotlin 이디엄 포팅
 * 중위 표기 → 후위 표기 변환 후 계산한다.
 *
 * 우선순위 체계: 숫자가 작을수록 수학적 우선순위가 높다 (* /=1, +-=2).
 * isPrior(스택 연산자, 입력 연산자): 스택 연산자를 팝하지 않아야 할 때 true 반환.
 */

private val NUMBER_CHARS = "0123456789.".toSet()

fun isNumber(c: Char): Boolean = c in NUMBER_CHARS

fun getPriority(operator: Char, inStack: Boolean): Int = when (operator) {
    '(' -> if (inStack) 3 else 0
    '*', '/' -> 1
    '+', '-' -> 2
    else -> -1
}

/** 스택 연산자의 우선순위가 입력 연산자보다 높으면(숫자가 크면) true를 반환한다. */
fun isPrior(opInStack: Char, opInToken: Char): Boolean =
    getPriority(opInStack, inStack = true) > getPriority(opInToken, inStack = false)

/** 중위 표기식 → 후위 표기식 변환 */
fun getPostfix(infix: String): String {
    val stack = ArrayDeque<String>()
    val postfix = StringBuilder()
    var i = 0

    while (i < infix.length) {
        val c = infix[i]
        when {
            isNumber(c) -> {
                val start = i
                while (i < infix.length && isNumber(infix[i])) i++
                postfix.append(infix.substring(start, i)).append(' ')
                continue                             // i++ 생략
            }
            c == ')' -> {
                while (stack.isNotEmpty()) {
                    val top = stack.removeLast()
                    if (top == "(") break
                    postfix.append(top)
                }
            }
            c == ' ' -> { /* 공백 무시 */ }
            else -> {
                // 연산자 또는 '('
                while (stack.isNotEmpty() && !isPrior(stack.last()[0], c)) {
                    val top = stack.removeLast()
                    if (top != "(") postfix.append(top)
                }
                stack.addLast(c.toString())
            }
        }
        i++
    }

    while (stack.isNotEmpty()) {
        val top = stack.removeLast()
        if (top != "(") postfix.append(top)
    }

    return postfix.toString()
}

/** C의 gcvt(value, 10, buf)와 동등: 유효숫자 10자리로 변환, 정수면 소수점 없이 반환. */
fun gcvt10(value: Double): String {
    if (value == 0.0) return "0"
    if (value.isInfinite() || value.isNaN()) return value.toString()
    val exp = Math.floor(Math.log10(Math.abs(value))).toInt()
    val decimals = maxOf(0, 9 - exp)
    val factor = Math.pow(10.0, decimals.toDouble())
    val rounded = Math.round(value * factor).toDouble() / factor
    return if (rounded == rounded.toLong().toDouble() && !rounded.isInfinite())
        rounded.toLong().toString()
    else
        rounded.toString()
}

/** 후위 표기식 계산 */
fun calculate(postfix: String): Double {
    val stack = ArrayDeque<String>()
    var i = 0

    while (i < postfix.length) {
        val c = postfix[i]
        when {
            c == ' ' -> { i++; continue }
            isNumber(c) -> {
                val start = i
                while (i < postfix.length && isNumber(postfix[i])) i++
                stack.addLast(postfix.substring(start, i))
                continue
            }
            c in "+-*/" -> {
                val b = stack.removeLast().trim().toDouble()
                val a = stack.removeLast().trim().toDouble()
                val result = when (c) {
                    '+' -> a + b
                    '-' -> a - b
                    '*' -> a * b
                    '/' -> a / b
                    else -> 0.0
                }
                stack.addLast(gcvt10(result))
            }
        }
        i++
    }

    return stack.last().trim().toDouble()
}

fun main() {
    val expressions = listOf(
        "3+4",
        "3+4*2",
        "(3+4)*2",
        "10/4",
        "1+2*3-4/2"
    )
    for (expr in expressions) {
        val postfix = getPostfix(expr)
        val result = calculate(postfix)
        println("$expr = $result  (postfix: $postfix)")
    }
}
