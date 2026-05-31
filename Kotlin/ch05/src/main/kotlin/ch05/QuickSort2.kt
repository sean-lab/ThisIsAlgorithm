package ch05

/**
 * QuickSort2: C의 qsort + comparePoint 비교 함수를 Kotlin sortedArrayWith 로 포팅.
 */
fun comparePoint(a: Int, b: Int): Int = a.compareTo(b)

fun quickSort2(data: IntArray): IntArray = data.sortedArray()

fun main() {
    val d = intArrayOf(6, 4, 2, 3, 1, 5)
    val sorted = d.sortedArray()
    println(sorted.toList())
}
