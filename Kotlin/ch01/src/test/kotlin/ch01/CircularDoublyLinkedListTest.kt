package ch01

import kotlin.test.*

class CircularDoublyLinkedListTest {

    @Test
    fun `append is circular`() {
        val list = CircularDoublyLinkedList()
        for (i in 0 until 5) list.append(i)
        assertEquals(5, list.count)
        // head.prev가 tail, tail.next가 head
        val h = list.getAt(0)!!
        assertEquals(4, h.prev!!.data)
        assertEquals(0, h.prev!!.next!!.data)
        // 원형 순회: 노드 수 이상을 넘어도 환형으로 이어진다
        assertEquals(listOf(0, 1, 2, 3, 4, 0, 1), list.forward(7))
    }

    @Test
    fun `remove middle keeps circular`() {
        val list = CircularDoublyLinkedList()
        for (i in 0 until 5) list.append(i)
        val node = list.getAt(2)!!
        list.remove(node)
        assertEquals(4, list.count)
        assertEquals(listOf(0, 1, 3, 4), list.forward(4))
    }

    @Test
    fun `remove head advances head`() {
        val list = CircularDoublyLinkedList()
        for (i in 0 until 3) list.append(i)
        val head = list.getAt(0)!!
        list.remove(head)
        assertEquals(2, list.count)
        assertEquals(1, list.getAt(0)!!.data)
        assertEquals(listOf(1, 2, 1), list.forward(3))
    }

    @Test
    fun `remove last node empties list`() {
        val list = CircularDoublyLinkedList()
        list.append(42)
        list.remove(list.getAt(0)!!)
        assertEquals(0, list.count)
        assertNull(list.head)
    }

    @Test
    fun `getAt bounds check`() {
        val list = CircularDoublyLinkedList()
        for (i in 0 until 3) list.append(i)
        assertNull(list.getAt(-1))
        assertEquals(1, list.getAt(1)!!.data)
    }
}
