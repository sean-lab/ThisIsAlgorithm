package ch01

import kotlin.test.*

class LinkedListTest {

    @Test
    fun `append builds list in order`() {
        val list = LinkedList()
        for (i in 0 until 5) list.append(i)
        assertEquals(5, list.count)
        assertEquals(listOf(0, 1, 2, 3, 4), list.toList())
    }

    @Test
    fun `insertHead prepends node`() {
        val list = LinkedList()
        for (i in 0 until 3) list.append(i)
        list.insertHead(-1)
        assertEquals(listOf(-1, 0, 1, 2), list.toList())
    }

    @Test
    fun `insertAfter inserts in middle`() {
        val list = LinkedList()
        for (i in 0 until 3) list.append(i)
        val second = list.getAt(1)!!
        list.insertAfter(second, 99)
        assertEquals(listOf(0, 1, 99, 2), list.toList())
    }

    @Test
    fun `remove head and middle`() {
        val list = LinkedList()
        for (i in 0 until 4) list.append(i)
        // 헤드 제거
        list.remove(list.getAt(0)!!)
        assertEquals(listOf(1, 2, 3), list.toList())
        // 가운데 제거
        list.remove(list.getAt(1)!!)
        assertEquals(listOf(1, 3), list.toList())
    }

    @Test
    fun `getAt out of range returns null`() {
        val list = LinkedList()
        list.append(7)
        assertNull(list.getAt(-1))
        assertNull(list.getAt(5))
    }
}
