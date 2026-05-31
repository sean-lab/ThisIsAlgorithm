package ch12

/**
 * 분할 정복 피보나치 (Fibonacci via Matrix Power)
 * Rust/ch12/src/fibonacci_dnc.rs → Kotlin 이디엄 포팅
 */
typealias Mat2 = Array<LongArray>

private fun mat2Mul(a: Mat2, b: Mat2): Mat2 = arrayOf(
    longArrayOf(
        a[0][0] * b[0][0] + a[0][1] * b[1][0],
        a[0][0] * b[1][0] + a[0][1] * b[1][1]
    ),
    longArrayOf(
        a[1][0] * b[0][0] + a[1][1] * b[1][0],
        a[1][0] * b[1][0] + a[1][1] * b[1][1]
    )
)

private fun mat2Pow(a: Mat2, n: Int): Mat2 {
    if (n <= 1) return a
    var result = mat2Pow(a, n / 2)
    result = mat2Mul(result, result)
    if (n and 1 != 0) {
        result = mat2Mul(result, arrayOf(longArrayOf(1, 1), longArrayOf(1, 0)))
    }
    return result
}

fun fibonacciDnC(n: Int): Long {
    val a: Mat2 = arrayOf(longArrayOf(1, 1), longArrayOf(1, 0))
    return mat2Pow(a, n)[0][1]
}

fun main() {
    println(fibonacciDnC(10))   // 55
    println(fibonacciDnC(46))   // 1836311903
}
