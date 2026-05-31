package ch10

/**
 * 카프-라빈 (Karp-Rabin)
 * Rust/ch10/src/karp_rabin.rs → Kotlin 이디엄 포팅
 * C char 는 signed byte 이므로 ByteArray 의 signed toInt() 그대로 사용.
 */
fun krHash(string: ByteArray, size: Int): Int {
    var hashValue = 0
    for (i in 0 until size) {
        hashValue = string[i].toInt() + hashValue * 2
    }
    return hashValue
}

fun krReHash(string: ByteArray, start: Int, size: Int, hashPrev: Int, coefficient: Int): Int {
    if (start == 0) return hashPrev
    val last = string[start + size - 1].toInt()
    val first = string[start - 1].toInt()
    return last + (hashPrev - coefficient * first) * 2
}

fun karpRabin(text: ByteArray, start: Int, pattern: ByteArray): Int {
    val n = text.size; val m = pattern.size
    val coefficient = 1 shl (m - 1)
    var hashText = krHash(text, m)
    val hashPattern = krHash(pattern, m)
    var i = start
    while (i <= n - m) {
        hashText = krReHash(text, i, m, hashText, coefficient)
        if (hashPattern == hashText) {
            var j = 0
            while (j < m && text[i + j] == pattern[j]) j++
            if (j >= m) return i
        }
        i++
    }
    return -1
}

fun karpRabin(text: String, pattern: String) =
    karpRabin(text.toByteArray(), 0, pattern.toByteArray())

fun main() {
    println(karpRabin("hello world", "world"))
    println(karpRabin("abcabcabd", "abcabd"))
    println(karpRabin("hello", "xyz"))
}
