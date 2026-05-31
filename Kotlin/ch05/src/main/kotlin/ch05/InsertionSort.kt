package ch05

fun insertionSort(data: IntArray) {
    val n = data.size
    for (i in 1 until n) {
        if (data[i - 1] <= data[i]) continue
        val value = data[i]
        for (j in 0 until i) {
            if (data[j] > value) {
                // memmove: [j..i) 를 한 칸 오른쪽으로 밀기
                System.arraycopy(data, j, data, j + 1, i - j)
                data[j] = value
                break
            }
        }
    }
}

fun main() {
    val d = intArrayOf(6, 4, 2, 3, 1, 5)
    insertionSort(d)
    println(d.toList())
}
