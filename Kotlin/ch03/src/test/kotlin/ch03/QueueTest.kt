package ch03

import kotlin.test.*

class CircularQueueTest {

    @Test
    fun `enqueue dequeue is FIFO`() {
        val q = CircularQueue(10)
        q.enqueue(1); q.enqueue(2); q.enqueue(3)
        assertEquals(3, q.getSize())
        assertEquals(1, q.dequeue())
        assertEquals(2, q.dequeue())
        assertEquals(3, q.dequeue())
        assertTrue(q.isEmpty())
    }

    @Test
    fun `fills to capacity`() {
        val q = CircularQueue(3)
        assertTrue(q.isEmpty())
        q.enqueue(10); q.enqueue(20); q.enqueue(30)
        assertTrue(q.isFull())
        assertEquals(3, q.getSize())
    }

    @Test
    fun `wraps around`() {
        val q = CircularQueue(3)
        q.enqueue(1); q.enqueue(2); q.enqueue(3)
        assertEquals(1, q.dequeue())
        assertEquals(2, q.dequeue())
        q.enqueue(4); q.enqueue(5)
        assertTrue(q.isFull())
        assertEquals(3, q.dequeue())
        assertEquals(4, q.dequeue())
        assertEquals(5, q.dequeue())
        assertTrue(q.isEmpty())
    }
}

class LinkedQueueTest {

    @Test
    fun `enqueue dequeue is FIFO`() {
        val q = LinkedQueue()
        q.enqueue("abc"); q.enqueue("def"); q.enqueue("efg")
        assertEquals(3, q.count)
        assertEquals("abc", q.dequeue())
        assertEquals("def", q.dequeue())
        assertEquals("efg", q.dequeue())
        assertTrue(q.isEmpty())
        assertEquals(0, q.count)
    }

    @Test
    fun `dequeue empty is null`() {
        val q = LinkedQueue()
        assertTrue(q.isEmpty())
        assertNull(q.dequeue())
        assertEquals(0, q.count)
    }
}
