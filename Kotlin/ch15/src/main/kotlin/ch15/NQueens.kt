package ch15

/**
 * N-Queens (백트래킹)
 * Rust/ch15/src/n_queens.rs → Kotlin 이디엄 포팅
 */
fun isThreatened(columns: IntArray, newRow: Int): Boolean {
    for (row in 0 until newRow) {
        if (columns[newRow] == columns[row] ||
            Math.abs(columns[newRow] - columns[row]) == Math.abs(newRow - row))
            return true
    }
    return false
}

private fun countQueens(columns: IntArray, row: Int, n: Int, count: IntArray) {
    if (isThreatened(columns, row)) return
    if (row == n - 1) { count[0]++; return }
    for (i in 0 until n) {
        columns[row + 1] = i
        countQueens(columns, row + 1, n, count)
    }
}

fun countSolutions(n: Int): Int {
    val columns = IntArray(n)
    val count = IntArray(1)
    for (i in 0 until n) {
        columns[0] = i
        countQueens(columns, 0, n, count)
    }
    return count[0]
}

fun main() {
    println("4-queens: ${countSolutions(4)}")    // 2
    println("8-queens: ${countSolutions(8)}")    // 92
}
