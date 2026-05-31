package ch13

/**
 * LCS 동적 계획법 + 역추적 (Longest Common Subsequence - DP with traceback)
 * Rust/ch13/src/lcsdp.rs → Kotlin 이디엄 포팅
 */
fun lcsdp(x: ByteArray, y: ByteArray, table: Array<IntArray>): Int {
    for (m in 0..x.size) table[m][0] = 0
    for (n in 0..y.size) table[0][n] = 0
    for (m in 1..x.size) {
        for (n in 1..y.size) {
            table[m][n] = when {
                x[m - 1] == y[n - 1]          -> table[m - 1][n - 1] + 1
                table[m][n - 1] >= table[m - 1][n] -> table[m][n - 1]
                else                           -> table[m - 1][n]
            }
        }
    }
    return table[x.size][y.size]
}

fun lcsTraceBack(x: ByteArray, y: ByteArray, m: Int, n: Int, table: Array<IntArray>, result: StringBuilder) {
    if (m == 0 || n == 0) return
    when {
        table[m][n] > table[m][n - 1] && table[m][n] > table[m - 1][n] && table[m][n] > table[m - 1][n - 1] -> {
            lcsTraceBack(x, y, m - 1, n - 1, table, result)
            result.append(x[m - 1].toInt().toChar())
        }
        table[m][n] > table[m - 1][n] && table[m][n] == table[m][n - 1] ->
            lcsTraceBack(x, y, m, n - 1, table, result)
        else ->
            lcsTraceBack(x, y, m - 1, n, table, result)
    }
}

fun main() {
    val x = "GOOD MORNING.".toByteArray()
    val y = "GUTEN MORGEN.".toByteArray()
    val table = Array(x.size + 1) { IntArray(y.size + 1) }
    val len = lcsdp(x, y, table)
    val sb = StringBuilder()
    lcsTraceBack(x, y, x.size, y.size, table, sb)
    println("LCS length: $len, LCS: $sb")   // 7, "G MORN."
}
