package ch03

/**
 * 환형 큐 (Circular Queue)
 * Rust/ch03/src/circular_queue.rs → Kotlin 이디엄 포팅
 * capacity+1 개의 슬롯과 front/rear 인덱스를 사용한다.
 */
class CircularQueue(val capacity: Int) {

    private val nodes = IntArray(capacity + 1)
    var front = 0
        private set
    var rear = 0
        private set

    fun enqueue(data: Int) {
        val position = rear
        rear = if (rear == capacity) 0 else rear + 1
        nodes[position] = data
    }

    fun dequeue(): Int {
        val position = front
        front = if (front == capacity) 0 else front + 1
        return nodes[position]
    }

    fun getSize(): Int =
        if (front <= rear) rear - front
        else rear + (capacity - front) + 1

    fun isEmpty(): Boolean = front == rear

    fun isFull(): Boolean =
        if (front < rear) (rear - front) == capacity
        else (rear + 1) == front
}

fun main() {
    val q = CircularQueue(10)
    for (i in 1..5) q.enqueue(i * 10)

    println("Size: ${q.getSize()}")
    while (!q.isEmpty()) println("Dequeued: ${q.dequeue()}")
}
