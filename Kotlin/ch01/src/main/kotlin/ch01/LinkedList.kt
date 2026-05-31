package ch01

/**
 * 단순 연결 리스트 (Singly Linked List)
 * Rust/ch01/src/linked_list.rs → Kotlin 이디엄 포팅
 */
class LinkedList {

    class Node(val data: Int) {
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
        }
        return node
    }

    /** 현재 노드 뒤에 새 노드를 삽입한다. */
    fun insertAfter(current: Node, data: Int): Node {
        val node = Node(data)
        node.next = current.next
        current.next = node
        return node
    }

    /** 리스트 앞에 새 헤드 노드를 삽입한다. */
    fun insertHead(data: Int): Node {
        val node = Node(data)
        node.next = head
        head = node
        return node
    }

    /** 노드를 리스트에서 분리한다 (메모리는 GC가 해제). */
    fun remove(node: Node) {
        if (head === node) {
            head = node.next
        } else {
            var current = head
            while (current?.next !== node) current = current?.next
            current?.next = node.next
        }
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

    /** 리스트를 Int 리스트로 변환 */
    fun toList(): List<Int> = buildList {
        var cur = head
        while (cur != null) { add(cur.data); cur = cur.next }
    }
}

fun main() {
    val list = LinkedList()

    // 노드 5개 추가
    for (i in 0 until 5) list.append(i)
    list.insertHead(-1)
    list.insertHead(-2)

    // 리스트 출력
    for (i in 0 until list.count) println("List[$i] : ${list.getAt(i)!!.data}")

    // 리스트의 세 번째 노드 뒤에 새 노드 삽입
    println("\nInserting 3000 After [2]...\n")
    list.insertAfter(list.getAt(2)!!, 3000)

    for (i in 0 until list.count) println("List[$i] : ${list.getAt(i)!!.data}")

    // 모든 노드를 리스트에서 제거
    println("\nDestroying List...")
    while (list.count > 0) list.remove(list.getAt(0)!!)
}
