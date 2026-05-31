package ch07

/**
 * 최소 힙 (Min-Heap)
 * Rust/ch07/src/heap.rs → Kotlin 이디엄 포팅
 */
class Heap(initialCapacity: Int = 4) {
    private val nodes = ArrayList<Int>(initialCapacity)
    val usedSize get() = nodes.size

    fun insert(data: Int) {
        nodes.add(data)
        var pos = nodes.size - 1
        while (pos > 0) {
            val parent = (pos - 1) / 2
            if (nodes[pos] < nodes[parent]) {
                swap(pos, parent)
                pos = parent
            } else break
        }
    }

    fun deleteMin(): Int {
        check(nodes.isNotEmpty()) { "Heap is empty" }
        val root = nodes[0]
        nodes[0] = nodes[nodes.size - 1]
        nodes.removeAt(nodes.size - 1)
        siftDown(0)
        return root
    }

    private fun siftDown(startPos: Int) {
        var pos = startPos
        while (true) {
            val left = 2 * pos + 1
            val right = left + 1
            if (left >= nodes.size) break
            val selected = when {
                right >= nodes.size -> left
                nodes[left] > nodes[right] -> right
                else -> left
            }
            if (nodes[selected] < nodes[pos]) {
                swap(pos, selected)
                pos = selected
            } else break
        }
    }

    private fun swap(i: Int, j: Int) {
        val tmp = nodes[i]; nodes[i] = nodes[j]; nodes[j] = tmp
    }
}

fun main() {
    val h = Heap(3)
    listOf(12, 87, 111, 34, 16, 75).forEach { h.insert(it) }
    val out = mutableListOf<Int>()
    while (h.usedSize > 0) out.add(h.deleteMin())
    println(out)
}
