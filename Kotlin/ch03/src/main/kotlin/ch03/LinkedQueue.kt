package ch03

/**
 * 연결 리스트 기반 큐 (Linked Queue)
 * Rust/ch03/src/linked_queue.rs → Kotlin 이디엄 포팅
 * 책의 원본 C 코드는 char* 데이터를 저장하므로 String을 저장한다.
 */
class LinkedQueue {

    private val items = ArrayDeque<String>()
    var count = 0
        private set

    fun enqueue(data: String) {
        items.addLast(data)
        count++
    }

    fun dequeue(): String? {
        val front = items.removeFirstOrNull()
        if (front != null) count--
        return front
    }

    fun isEmpty(): Boolean = items.isEmpty()
}

fun main() {
    val q = LinkedQueue()
    q.enqueue("abc")
    q.enqueue("def")
    q.enqueue("efg")

    println("Count: ${q.count}")
    while (!q.isEmpty()) println("Dequeued: ${q.dequeue()}")
}
