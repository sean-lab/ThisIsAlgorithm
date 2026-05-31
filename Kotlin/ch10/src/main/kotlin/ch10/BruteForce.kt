package ch10

/**
 * 무차별 대입 문자열 탐색 (Brute Force)
 * Rust/ch10/src/brute_force.rs → Kotlin 이디엄 포팅
 */
fun bruteForce(text: ByteArray, start: Int, pattern: ByteArray): Int {
    val n = text.size; val m = pattern.size
    var i = start
    while (i <= n - m) {
        var j = 0
        while (j < m && text[i + j] == pattern[j]) j++
        if (j >= m) return i
        i++
    }
    return -1
}

fun bruteForce(text: String, pattern: String) =
    bruteForce(text.toByteArray(), 0, pattern.toByteArray())

fun main() {
    println(bruteForce("hello world", "world"))
    println(bruteForce("abcabcabd", "abcabd"))
    println(bruteForce("hello", "xyz"))
}
