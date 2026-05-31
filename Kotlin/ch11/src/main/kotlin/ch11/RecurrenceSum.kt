package ch11

/**
 * 재귀적 합계 (Recurrence Sum)
 * Rust/ch11/src/recurrence_sum.rs → Kotlin 이디엄 포팅
 */
fun recurrenceSum(data: IntArray, from: Int = 0): Int =
    if (from == data.lastIndex) data[from]
    else data[from] + recurrenceSum(data, from + 1)

fun main() {
    val data = IntArray(150) { it + 1 }
    println(recurrenceSum(data, 0))   // 1..150 합
    println(recurrenceSum(data, 0).let { /* 1+2+...+150 = 11325 */ it })
    // 1..55 합
    val partial = IntArray(55) { it + 1 }
    println(recurrenceSum(partial))   // 1540
}
