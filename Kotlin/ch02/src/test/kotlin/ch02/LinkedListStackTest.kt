package ch02

import kotlin.test.*

class LinkedListStackTest {

    @Test
    fun `push pop is LIFO`() {
        val s = LinkedListStack()
        s.push("abc"); s.push("def"); s.push("efg")
        assertEquals(3, s.size())
        assertEquals("efg", s.top())
        assertEquals("efg", s.pop())
        assertEquals("def", s.pop())
        assertEquals("abc", s.pop())
        assertTrue(s.isEmpty())
    }

    @Test
    fun `pop empty is null`() {
        val s = LinkedListStack()
        assertTrue(s.isEmpty())
        assertNull(s.pop())
        assertNull(s.top())
    }
}
