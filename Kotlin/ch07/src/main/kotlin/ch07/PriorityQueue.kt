package ch07

/**
 * 우선순위 큐 (Priority Queue) - 최소 힙 기반
 * Rust/ch07/src/priority_queue.rs → Kotlin 이디엄 포팅
 */
data class PQNode(val priority: Int, val data: String)

class PriorityQueue(initialCapacity: Int = 4) {
    private val nodes = ArrayList<PQNode>(initialCapacity)
    val usedSize get() = nodes.size
    fun isEmpty() = nodes.isEmpty()

    fun enqueue(node: PQNode) {
        nodes.add(node)
        var pos = nodes.size - 1
        while (pos > 0) {
            val parent = (pos - 1) / 2
            if (nodes[pos].priority < nodes[parent].priority) {
                swap(pos, parent)
                pos = parent
            } else break
        }
    }

    fun dequeue(): PQNode {
        check(nodes.isNotEmpty()) { "PriorityQueue is empty" }
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
                nodes[left].priority > nodes[right].priority -> right
                else -> left
            }
            if (nodes[selected].priority < nodes[pos].priority) {
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
    val pq = PriorityQueue(3)
    listOf(
        PQNode(34, "코딩"), PQNode(12, "고객미팅"), PQNode(87, "커피타기"),
        PQNode(45, "문서작성"), PQNode(35, "디버깅"), PQNode(66, "이닦기")
    ).forEach { pq.enqueue(it) }
    while (!pq.isEmpty()) {
        val n = pq.dequeue()
        println("[${n.priority}] ${n.data}")
    }
}
