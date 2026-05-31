package ch12

/**
 * 빠른 거듭제곱 (Fast Exponentiation)
 * Rust/ch12/src/fast_exponentiation.rs → Kotlin 이디엄 포팅
 */
fun power(base: Int, exponent: Int): Long {
    if (exponent == 1) return base.toLong()
    if (base == 0) return 1L
    return if (exponent % 2 == 0) {
        val half = power(base, exponent / 2)
        half * half
    } else {
        val half = power(base, (exponent - 1) / 2)
        half * half * base.toLong()
    }
}

fun main() {
    println(power(2, 10))   // 1024
    println(power(2, 30))   // 1073741824
}
