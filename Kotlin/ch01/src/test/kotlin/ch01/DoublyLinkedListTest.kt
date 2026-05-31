package ch01

import kotlin.test.*

class DoublyLinkedListTest {

    @Test
    fun `append links both directions`() {
        val list = DoublyLinkedList()
        for (i in 0 until 5) list.append(i)
        assertEquals(listOf(0, 1, 2, 3, 4), list.toList())
        assertEquals(listOf(4, 3, 2, 1, 0), list.toListReverse())
    }

    @Test
    fun `insertAfter fixes prev links`() {
        val list = DoublyLinkedList()
        for (i in 0 until 3) list.append(i)
        list.insertAfter(list.getAt(1)!!, 99)
        assertEquals(listOf(0, 1, 99, 2), list.toList())
        assertEquals(listOf(2, 99, 1, 0), list.toListReverse())
    }

    @Test
    fun `remove head keeps links`() {
        val list = DoublyLinkedList()
        for (i in 0 until 4) list.append(i)
        list.remove(list.getAt(0)!!)
        assertEquals(listOf(1, 2, 3), list.toList())
        assertEquals(listOf(3, 2, 1), list.toListReverse())
    }

    @Test
    fun `remove middle keeps links`() {
        val list = DoublyLinkedList()
        for (i in 0 until 4) list.append(i)
        list.remove(list.getAt(1)!!)
        assertEquals(listOf(0, 2, 3), list.toList())
        assertEquals(listOf(3, 2, 0), list.toListReverse())
    }
}
