package ch06

/**
 * 이진 탐색 (Binary Search)
 * Rust/ch06/src/search.rs → Kotlin 이디엄 포팅
 */
data class Point(val id: Int, val point: Double)

fun binarySearch(list: List<Point>, target: Double): Point? {
    var left = 0
    var right = list.size - 1
    while (left <= right) {
        val mid = (left + right) / 2
        when {
            target == list[mid].point -> return list[mid]
            target > list[mid].point -> left = mid + 1
            else -> right = mid - 1
        }
    }
    return null
}

fun main() {
    val data = listOf(
        Point(1, 94.73), Point(2, 121.44), Point(3, 176.23),
        Point(4, 224.72), Point(5, 671.78), Point(6, 877.88)
    ) // 이미 point 기준 정렬됨
    val found = binarySearch(data, 671.78)
    println("Found: id=${found?.id}, point=${found?.point}")
}
