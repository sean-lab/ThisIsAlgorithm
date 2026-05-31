package ch06

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull

class SequentialSearchTest {
    private fun buildList(values: List<Int>) = SequentialSearchList().also {
        values.forEach { v -> it.append(v) }
    }

    @Test
    fun searchFindsAndMisses() {
        val list = buildList(listOf(1, 2, 6, 10, 4))
        assertEquals(6, list.search(6)?.data)
        assertNull(list.search(99))
    }

    @Test
    fun moveToFrontMovesMatchToHead() {
        val list = buildList(listOf(1, 2, 6, 10, 4))
        val matched = list.moveToFront(10)
        assertEquals(10, matched?.data)
        assertEquals(listOf(10, 1, 2, 6, 4), list.toList())
    }

    @Test
    fun transposeSwapsWithPrevious() {
        val list = buildList(listOf(1, 2, 6, 10, 4))
        val matched = list.transpose(10)
        assertEquals(10, matched?.data)
        assertEquals(listOf(1, 2, 10, 6, 4), list.toList())
    }
}

class BinarySearchTest {
    private val sortedData = listOf(
        Point(6, 13.88), Point(7, 25.95), Point(8, 27.13),
        Point(1, 94.73), Point(2, 121.44), Point(3, 176.23),
        Point(4, 224.72), Point(5, 671.78), Point(9, 877.88)
    )

    @Test
    fun findsExistingValue() {
        val found = binarySearch(sortedData, 671.78)
        assertNotNull(found)
        assertEquals(671.78, found!!.point)
    }

    @Test
    fun returnsNullForMissingValue() {
        assertNull(binarySearch(sortedData, -1.0))
    }
}

class BinarySearchTreeTest {
    private fun buildTree(): BinarySearchTree {
        val bst = BinarySearchTree()
        bst.insert(123)
        listOf(22, 9918, 424, 17, 3, 98, 34, 760, 317, 1).forEach { bst.insert(it) }
        return bst
    }

    @Test
    fun searchFindsAndMisses() {
        val bst = buildTree()
        assertEquals(17, bst.search(17)?.data)
        assertNull(bst.search(117))
    }

    @Test
    fun inorderIsSorted() {
        assertEquals(listOf(1, 3, 17, 22, 34, 98, 123, 317, 424, 760, 9918), buildTree().inorder())
    }

    @Test
    fun removeKeepsOrder() {
        val bst = buildTree()
        assertNotNull(bst.remove(98))
        assertEquals(listOf(1, 3, 17, 22, 34, 123, 317, 424, 760, 9918), bst.inorder())
    }
}

class RedBlackTreeTest {
    // 전역 RBT_NIL 센티넬을 공유하므로 모든 검증을 하나의 테스트에 모았다.
    @Test
    fun insertSearchRemoveMaintainsInvariants() {
        val rbt = RedBlackTree()
        listOf(10, 20, 30, 15, 25, 5, 1, 40, 35, 50).forEach {
            rbt.insert(RBTNode(it))
        }

        // 루트는 항상 검정색이어야 한다.
        assertEquals(RBT_BLACK, rbt.root!!.color)

        // 중위 순회는 정렬된 순서여야 한다.
        assertEquals(listOf(1, 5, 10, 15, 20, 25, 30, 35, 40, 50), rbt.inorder())

        // 검색이 동작한다.
        assertEquals(25, rbt.search(25)?.data)
        assertNull(rbt.search(999))

        // 삭제 후에도 정렬/루트 검정 불변식이 유지된다.
        assertNotNull(rbt.remove(20))
        assertEquals(RBT_BLACK, rbt.root!!.color)
        assertEquals(listOf(1, 5, 10, 15, 25, 30, 35, 40, 50), rbt.inorder())
    }
}
