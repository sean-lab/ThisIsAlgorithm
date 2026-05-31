package ch01

/**
 * 이중 연결 리스트 (Doubly Linked List)
 * Rust/ch01/src/doubly_linked_list.rs → Kotlin 이디엄 포팅
 */
class DoublyLinkedList {

    class Node(val data: Int) {
        var prev: Node? = null
        var next: Node? = null
    }

    private var head: Node? = null

    /** 리스트 끝에 노드를 추가한다. */
    fun append(data: Int): Node {
        val node = Node(data)
        if (head == null) {
            head = node
        } else {
            var tail = head!!
            while (tail.next != null) tail = tail.next!!
            tail.next = node
            node.prev = tail
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
        if (head === node) {
            head = node.next
            head?.prev = null
        } else {
            node.prev?.next = node.next
            node.next?.prev = node.prev
        }
        node.prev = null
        node.next = null
    }

    /** index 위치의 노드를 반환한다. 범위를 벗어나면 null. */
    fun getAt(index: Int): Node? {
        if (index < 0) return null
        var current = head
        repeat(index) { current = current?.next }
        return current
    }

    /** 노드 개수 */
    val count: Int
        get() {
            var n = 0
            var cur = head
            while (cur != null) { n++; cur = cur.next }
            return n
        }

    /** 리스트를 Int 리스트로 변환 (정방향) */
    fun toList(): List<Int> = buildList {
        var cur = head
        while (cur != null) { add(cur.data); cur = cur.next }
    }

    /** 리스트를 Int 리스트로 변환 (역방향 — prev 링크 검증용) */
    fun toListReverse(): List<Int> {
        if (head == null) return emptyList()
        var tail = head!!
        while (tail.next != null) tail = tail.next!!
        return buildList {
            var cur: Node? = tail
            while (cur != null) { add(cur.data); cur = cur.prev }
        }
    }
}

fun main() {
    val list = DoublyLinkedList()

    for (i in 0 until 5) list.append(i)
    for (i in 0 until list.count) println("List[$i] : ${list.getAt(i)!!.data}")

    println("\nInserting 3000 After [2]...\n")
    list.insertAfter(list.getAt(2)!!, 3000)

    for (i in 0 until list.count) println("List[$i] : ${list.getAt(i)!!.data}")

    println("\nDestroying List...")
    while (list.count > 0) list.remove(list.getAt(0)!!)
}
