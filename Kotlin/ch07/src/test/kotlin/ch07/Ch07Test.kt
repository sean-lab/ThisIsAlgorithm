package ch07

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class HeapTest {
    @Test
    fun deleteMinReturnsAscending() {
        val h = Heap(3)
        listOf(12, 87, 111, 34, 16, 75).forEach { h.insert(it) }
        assertEquals(6, h.usedSize)

        val out = mutableListOf<Int>()
        repeat(6) { out.add(h.deleteMin()) }
        assertEquals(listOf(12, 16, 34, 75, 87, 111), out)
        assertEquals(0, h.usedSize)
    }

    @Test
    fun rootIsMinimumAfterInserts() {
        val h = Heap(3)
        listOf(50, 30, 70, 10, 90).forEach { h.insert(it) }
        assertEquals(10, h.deleteMin())
    }
}

class PriorityQueueTest {
    @Test
    fun dequeueReturnsPriorityAscending() {
        val pq = PriorityQueue(3)
        listOf(
            PQNode(34, "코딩"), PQNode(12, "고객미팅"), PQNode(87, "커피타기"),
            PQNode(45, "문서작성"), PQNode(35, "디버깅"), PQNode(66, "이닦기")
        ).forEach { pq.enqueue(it) }
        assertEquals(6, pq.usedSize)

        val out = mutableListOf<Pair<Int, String>>()
        while (!pq.isEmpty()) {
            val n = pq.dequeue()
            out.add(Pair(n.priority, n.data))
        }
        assertEquals(
            listOf(
                12 to "고객미팅", 34 to "코딩", 35 to "디버깅",
                45 to "문서작성", 66 to "이닦기", 87 to "커피타기"
            ),
            out
        )
        assertTrue(pq.isEmpty())
    }
}
