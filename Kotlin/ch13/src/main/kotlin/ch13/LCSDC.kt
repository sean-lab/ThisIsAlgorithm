package ch13

/**
 * LCS 분할 정복 (Longest Common Subsequence - Divide and Conquer / recursive)
 * Rust/ch13/src/lcsdc.rs → Kotlin 이디엄 포팅
 */
fun lcsdc(x: ByteArray, y: ByteArray, i: Int, j: Int, table: Array<IntArray>): Int {
    if (i == 0 || j == 0) {
        table[i][j] = 0
        return 0
    }
    if (x[i - 1] == y[j - 1]) {
        table[i][j] = lcsdc(x, y, i - 1, j - 1, table) + 1
        return table[i][j]
    }
    val a = lcsdc(x, y, i - 1, j, table)
    val b = lcsdc(x, y, i, j - 1, table)
    table[i][j] = maxOf(a, b)
    return table[i][j]
}

fun main() {
    val x = "GOOD MORNING.".toByteArray()
    val y = "GUTEN MORGEN.".toByteArray()
    val table = Array(x.size + 1) { IntArray(y.size + 1) }
    println(lcsdc(x, y, x.size, y.size, table))   // 7
}
