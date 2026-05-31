package ch14

/**
 * 거스름돈 만들기 (Making Change, Greedy)
 * Rust/ch14/src/making_change.rs → Kotlin 이디엄 포팅
 */
fun countCoins(amount: Int, coinUnit: Int): Int {
    var count = 0
    var remaining = amount
    while (remaining >= coinUnit) {
        count++
        remaining -= coinUnit
    }
    return count
}

fun getChange(price: Int, pay: Int, coinUnits: IntArray, change: IntArray) {
    var remaining = pay - price
    for (i in coinUnits.indices) {
        change[i] = countCoins(remaining, coinUnits[i])
        remaining -= coinUnits[i] * change[i]
    }
}

fun main() {
    val units = intArrayOf(1000, 500, 100, 50, 10)
    val change = IntArray(5)
    getChange(7400, 10000, units, change)
    units.zip(change.toList()).forEach { (u, c) -> println("${u}원 : ${c}개") }
}
