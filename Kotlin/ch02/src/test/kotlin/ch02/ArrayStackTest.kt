package ch02

import kotlin.test.*

class ArrayStackTest {

    @Test
    fun `push pop is LIFO`() {
        val s = ArrayStack(10)
        s.push(3); s.push(37); s.push(11)
        assertEquals(3, s.getSize())
        assertEquals(11, s.top())
        assertEquals(11, s.pop())
        assertEquals(37, s.pop())
        assertEquals(3, s.pop())
        assertTrue(s.isEmpty())
    }

    @Test
    fun `overflow does not push`() {
        val s = ArrayStack(2)
        s.push(1); s.push(2); s.push(3)     // 용량 초과 → 무시
        assertEquals(2, s.getSize())
        assertEquals(2, s.top())
    }

    @Test
    fun `underflow returns zero`() {
        val s = ArrayStack(4)
        assertTrue(s.isEmpty())
        assertEquals(0, s.pop())
        assertEquals(0, s.top())
    }

    @Test
    fun `empty after creation`() {
        val s = ArrayStack(5)
        assertTrue(s.isEmpty())
        assertEquals(0, s.getSize())
        assertEquals(5, s.capacity)
    }
}
