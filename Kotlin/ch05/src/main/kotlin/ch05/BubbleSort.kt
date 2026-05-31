package ch05

fun bubbleSort(data: IntArray) {
    val n = data.size
    for (i in 0 until n - 1) {
        for (j in 0 until n - i - 1) {
            if (data[j] > data[j + 1]) {
                val tmp = data[j]; data[j] = data[j + 1]; data[j + 1] = tmp
            }
        }
    }
}

fun main() {
    val d = intArrayOf(6, 4, 2, 3, 1, 5)
    bubbleSort(d)
    println(d.toList())
}
