package ch06

/**
 * 순차 탐색 + 자기구성 탐색 (Move-To-Front, Transpose)
 * Rust/ch06/src/sequential_search.rs → Kotlin 이디엄 포팅
 */
class SLLNode(var data: Int) {
    var next: SLLNode? = null
}

class SequentialSearchList {
    var head: SLLNode? = null

    fun append(data: Int) {
        val newNode = SLLNode(data)
        if (head == null) {
            head = newNode
        } else {
            var tail = head!!
            while (tail.next != null) tail = tail.next!!
            tail.next = newNode
        }
    }

    fun search(target: Int): SLLNode? {
        var cur = head
        while (cur != null) {
            if (cur.data == target) return cur
            cur = cur.next
        }
        return null
    }

    /** 찾은 노드를 리스트 앞으로 이동한다. */
    fun moveToFront(target: Int): SLLNode? {
        var cur = head
        var previous: SLLNode? = null
        while (cur != null) {
            if (cur.data == target) {
                if (previous != null) {
                    previous.next = cur.next
                    cur.next = head
                    head = cur
                }
                return cur
            }
            previous = cur
            cur = cur.next
        }
        return null
    }

    /** 찾은 노드를 바로 앞 노드와 위치를 교환한다. */
    fun transpose(target: Int): SLLNode? {
        var cur = head
        var pprevious: SLLNode? = null
        var previous: SLLNode? = null
        while (cur != null) {
            if (cur.data == target) {
                if (previous != null) {
                    if (pprevious != null) pprevious.next = cur
                    else head = cur
                    previous.next = cur.next
                    cur.next = previous
                }
                return cur
            } else {
                if (previous != null) pprevious = previous
                previous = cur
                cur = cur.next
            }
        }
        return null
    }

    fun toList(): List<Int> = buildList {
        var cur = head
        while (cur != null) { add(cur.data); cur = cur.next }
    }
}

fun main() {
    val list = SequentialSearchList()
    listOf(1, 2, 6, 10, 4).forEach { list.append(it) }
    println("Search 6: ${list.search(6)?.data}")
    list.moveToFront(10)
    println("After MTF(10): ${list.toList()}")
    list.transpose(6)
    println("After Transpose(6): ${list.toList()}")
}
