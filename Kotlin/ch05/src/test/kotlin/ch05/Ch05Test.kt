package ch05

import kotlin.test.Test
import kotlin.test.assertContentEquals
import kotlin.test.assertEquals

class BubbleSortTest {
    @Test
    fun sortsSample() {
        val d = intArrayOf(6, 4, 2, 3, 1, 5)
        bubbleSort(d)
        assertContentEquals(intArrayOf(1, 2, 3, 4, 5, 6), d)
    }

    @Test
    fun handlesEdgeCases() {
        val empty = intArrayOf(); bubbleSort(empty); assertContentEquals(intArrayOf(), empty)
        val single = intArrayOf(42); bubbleSort(single); assertContentEquals(intArrayOf(42), single)
        val sorted = intArrayOf(1, 2, 3, 4); bubbleSort(sorted); assertContentEquals(intArrayOf(1, 2, 3, 4), sorted)
        val rev = intArrayOf(5, 4, 3, 2, 1); bubbleSort(rev); assertContentEquals(intArrayOf(1, 2, 3, 4, 5), rev)
    }

    @Test
    fun handlesDuplicatesAndNegatives() {
        val d = intArrayOf(3, -1, 3, 0, -1, 2)
        bubbleSort(d)
        assertContentEquals(intArrayOf(-1, -1, 0, 2, 3, 3), d)
    }
}

class InsertionSortTest {
    @Test
    fun sortsSample() {
        val d = intArrayOf(6, 4, 2, 3, 1, 5)
        insertionSort(d)
        assertContentEquals(intArrayOf(1, 2, 3, 4, 5, 6), d)
    }

    @Test
    fun handlesEdgeCases() {
        val empty = intArrayOf(); insertionSort(empty); assertContentEquals(intArrayOf(), empty)
        val single = intArrayOf(42); insertionSort(single); assertContentEquals(intArrayOf(42), single)
        val rev = intArrayOf(5, 4, 3, 2, 1); insertionSort(rev); assertContentEquals(intArrayOf(1, 2, 3, 4, 5), rev)
    }

    @Test
    fun handlesDuplicatesAndNegatives() {
        val d = intArrayOf(3, -1, 3, 0, -1, 2)
        insertionSort(d)
        assertContentEquals(intArrayOf(-1, -1, 0, 2, 3, 3), d)
    }
}

class QuickSortTest {
    @Test
    fun sortsSample() {
        val d = intArrayOf(6, 4, 2, 3, 1, 5)
        quickSortAll(d)
        assertContentEquals(intArrayOf(1, 2, 3, 4, 5, 6), d)
    }

    @Test
    fun handlesEdgeCases() {
        val empty = intArrayOf(); quickSortAll(empty); assertContentEquals(intArrayOf(), empty)
        val single = intArrayOf(42); quickSortAll(single); assertContentEquals(intArrayOf(42), single)
        val rev = intArrayOf(5, 4, 3, 2, 1); quickSortAll(rev); assertContentEquals(intArrayOf(1, 2, 3, 4, 5), rev)
    }

    @Test
    fun handlesDuplicatesAndNegatives() {
        val d = intArrayOf(3, -1, 3, 0, -1, 2)
        quickSortAll(d)
        assertContentEquals(intArrayOf(-1, -1, 0, 2, 3, 3), d)
    }

    @Test
    fun matchesStdSortOnManyInputs() {
        var seed = 0x12345678L
        repeat(200) {
            val n = (seed % 50).toInt().let { if (it < 0) -it else it }
            val v = IntArray(n) {
                seed = seed * 6364136223846793005L + 1
                ((seed ushr 33) % 100 - 50).toInt()
            }
            val expected = v.sorted().toIntArray()
            quickSortAll(v)
            assertContentEquals(expected, v)
        }
    }
}

class QuickSort2Test {
    @Test
    fun comparePointOrdersCorrectly() {
        assert(comparePoint(3, 5) < 0)
        assert(comparePoint(5, 3) > 0)
        assertEquals(0, comparePoint(4, 4))
    }

    @Test
    fun sortBySorts() {
        val d = intArrayOf(6, 4, 2, 3, 1, 5)
        assertContentEquals(intArrayOf(1, 2, 3, 4, 5, 6), quickSort2(d))
    }
}
