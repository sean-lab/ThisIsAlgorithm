package ch12

/**
 * 병합 정렬 (Merge Sort)
 * Rust/ch12/src/merge_sort.rs → Kotlin 이디엄 포팅
 */
fun mergeSort(data: IntArray, start: Int, end: Int) {
    if (end - start < 1) return
    val mid = (start + end) / 2
    mergeSort(data, start, mid)
    mergeSort(data, mid + 1, end)
    merge(data, start, mid, end)
}

private fun merge(data: IntArray, start: Int, middle: Int, end: Int) {
    val tmp = IntArray(end - start + 1)
    var left = start
    var right = middle + 1
    var dest = 0
    while (left <= middle && right <= end) {
        if (data[left] < data[right]) tmp[dest++] = data[left++]
        else tmp[dest++] = data[right++]
    }
    while (left <= middle) tmp[dest++] = data[left++]
    while (right <= end)   tmp[dest++] = data[right++]
    for (i in start..end) data[i] = tmp[i - start]
}

fun main() {
    val data = intArrayOf(334, 6, 4, 2, 3, 1, 5, 117, 12, 34)
    mergeSort(data, 0, data.lastIndex)
    println(data.toList())
}
