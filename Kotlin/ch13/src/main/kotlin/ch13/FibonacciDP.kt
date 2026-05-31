package ch13

/**
 * 동적 계획법 피보나치 (Fibonacci DP)
 * Rust/ch13/src/fibonacci_dp.rs → Kotlin 이디엄 포팅
 */
fun fibonacciDP(n: Int): Long {
    if (n == 0 || n == 1) return n.toLong()
    val table = LongArray(n + 1)
    table[0] = 0L
    table[1] = 1L
    for (i in 2..n) table[i] = table[i - 1] + table[i - 2]
    return table[n]
}

fun main() {
    println(fibonacciDP(10))   // 55
    println(fibonacciDP(46))   // 1836311903
}
