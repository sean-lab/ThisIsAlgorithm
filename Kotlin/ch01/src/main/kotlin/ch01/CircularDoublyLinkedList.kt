package ch01

/**
 * 원형 이중 연결 리스트 (Circular Doubly Linked List)
 * Rust/ch01/src/circular_doubly_linked_list.rs → Kotlin 이디엄 포팅
 */
class CircularDoublyLinkedList {

    class Node(val data: Int) {
        var prev: Node? = null
        var next: Node? = null
    }

    var head: Node? = null
        private set

    /** 리스트 끝에 노드를 추가한다 (테일과 헤드 사이에 삽입). */
    fun append(data: Int): Node {
        val node = Node(data)
        val h = head
        if (h == null) {
            node.next = node
            node.prev = node
            head = node
        } else {
            val tail = h.prev!!
            tail.next = node
            node.prev = tail
            node.next = h
            h.prev = node
        }
        return node
    }

    /** 현재 노드 뒤에 새 노드를 삽입한다. */
    fun insertAfter(current: Node, data: Int): Node {
        val node = Node(data)
        node.next = current.next
        node.prev = current
        current.next?.prev = node
        current.next = node
        return node
    }

    /** 노드를 리스트에서 분리한다. */
    fun remove(node: Node) {
        val prev = node.prev
        val next = node.next
        if (head === node) {
            head = if (next === node) null else next
        }
        prev?.next = next
        next?.prev = prev
        node.prev = null
        node.next = null
    }

    /**
     * index 위치의 노드를 반환한다.
     * index < 0 이면 null. index >= count 이면 원형 특성상 wrap 후 반환.
     */
    fun getAt(index: Int): Node? {
        if (index < 0 || head == null) return null
        var current = head!!
        var i = 0
        while (i < index) {
            current = current.next ?: return null
            i++
            if (current === head && i <= index) break
        }
        return current
    }

    /** 노드 개수 */
    val count: Int
        get() {
            val h = head ?: return 0
            var n = 0
            var cur = h
            do {
                n++
                cur = cur.next ?: return n
            } while (cur !== h)
            return n
        }

    /** 헤드부터 n개 노드를 순방향으로 수집한다 (원형이므로 count를 초과해도 가능). */
    fun forward(n: Int): List<Int> = buildList {
        val h = head ?: return@buildList
        var cur = h
        repeat(n) {
            add(cur.data)
            cur = cur.next ?: return@buildList
        }
    }
}

fun main() {
    val list = CircularDoublyLinkedList()

    for (i in 0 until 5) list.append(i)
    val count = list.count
    for (i in 0 until count) println("List[$i] : ${list.getAt(i)!!.data}")

    println("\nInserting 3000 After [2]...\n")
    list.insertAfter(list.getAt(2)!!, 3000)

    val newCount = list.count
    for (i in 0 until newCount) println("List[$i] : ${list.getAt(i)!!.data}")

    println("\nDestroying List...")
    while (list.count > 0) {
        val node = list.getAt(0)!!
        list.remove(node)
    }
}
