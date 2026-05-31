package ch10

/**
 * 커누스-모리스-프랫 (Knuth-Morris-Pratt)
 * Rust/ch10/src/knuth_morris_pratt.rs → Kotlin 이디엄 포팅
 */
fun kmpPreprocess(pattern: ByteArray): IntArray {
    val m = pattern.size
    val border = IntArray(m + 1)
    var i = 0; var j = -1
    border[0] = -1
    while (i < m) {
        while (j > -1 && pattern[i] != pattern[j]) j = border[j]
        i++; j++
        border[i] = j
    }
    return border
}

fun knuthMorrisPratt(text: ByteArray, start: Int, pattern: ByteArray): Int {
    val n = text.size; val m = pattern.size
    var i = start; var j = 0
    val border = kmpPreprocess(pattern)
    while (i < n) {
        while (j >= 0 && text[i] != pattern[j]) j = border[j]
        i++; j++
        if (j == m) return i - j
    }
    return -1
}

fun knuthMorrisPratt(text: String, pattern: String) =
    knuthMorrisPratt(text.toByteArray(), 0, pattern.toByteArray())

fun main() {
    println(knuthMorrisPratt("hello world", "world"))
    println(knuthMorrisPratt("abcabcabd", "abcabd"))
    println(knuthMorrisPratt("hello", "xyz"))
}
