package ch10

/**
 * 보이어-무어 (Boyer-Moore)
 * Rust/ch10/src/boyer_moore.rs → Kotlin 이디엄 포팅 (디버그 출력 제외)
 */

fun buildBadCharTable(pattern: ByteArray): IntArray {
    val table = IntArray(128) { -1 }
    for (j in pattern.indices) {
        table[pattern[j].toInt() and 0x7F] = j
    }
    return table
}

fun buildGoodSuffTable(pattern: ByteArray): Pair<IntArray, IntArray> {
    val m = pattern.size
    val posOfBorder = IntArray(m + 1)
    val goodSuff = IntArray(m + 1)

    // Case 1: 패턴 내 접두사-접미사 일치 위치 기반
    var i = m; var j = m + 1
    posOfBorder[i] = j
    while (i > 0) {
        while (j <= m && pattern[i - 1] != pattern[j - 1]) {
            if (goodSuff[j] == 0) goodSuff[j] = j - i
            j = posOfBorder[j]
        }
        i--; j--
        posOfBorder[i] = j
    }

    // Case 2: 패턴 전체 접두사 기반
    j = posOfBorder[0]
    for (k in 0..m) {
        if (goodSuff[k] == 0) goodSuff[k] = j
        if (k == j) j = posOfBorder[j]
    }

    return goodSuff to posOfBorder
}

fun boyerMoore(text: ByteArray, start: Int, pattern: ByteArray): Int {
    val n = text.size; val m = pattern.size
    val bct = buildBadCharTable(pattern)
    val (gst, _) = buildGoodSuffTable(pattern)
    var i = start
    while (i <= n - m) {
        var j = m - 1
        while (j >= 0 && pattern[j] == text[i + j]) j--
        if (j < 0) return i
        else i += maxOf(gst[j + 1], j - bct[text[i + j].toInt() and 0x7F])
    }
    return -1
}

fun boyerMoore(text: String, pattern: String) =
    boyerMoore(text.toByteArray(), 0, pattern.toByteArray())

fun main() {
    println(boyerMoore("hello world", "world"))
    println(boyerMoore("abcabcabd", "abcabd"))
    println(boyerMoore("hello", "xyz"))
}
