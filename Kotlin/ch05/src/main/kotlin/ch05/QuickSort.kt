package ch05

fun partition(data: IntArray, left: Int, right: Int): Int {
    val pivot = data[left]
    var l = left + 1
    var r = right

    while (l <= r) {
        while (l < r && data[l] <= pivot) l++
        while (l <= r && data[r] >= pivot) r--
        if (l < r) {
            val tmp = data[l]; data[l] = data[r]; data[r] = tmp
        } else break
    }

    val tmp = data[left]; data[left] = data[r]; data[r] = tmp
    return r
}

fun quickSort(data: IntArray, left: Int, right: Int) {
    if (left < right) {
        val idx = partition(data, left, right)
        quickSort(data, left, idx - 1)
        quickSort(data, idx + 1, right)
    }
}

fun quickSortAll(data: IntArray) {
    if (data.isEmpty()) return
    quickSort(data, 0, data.size - 1)
}

fun main() {
    val d = intArrayOf(6, 4, 2, 3, 1, 5)
    quickSortAll(d)
    println(d.toList())
}
