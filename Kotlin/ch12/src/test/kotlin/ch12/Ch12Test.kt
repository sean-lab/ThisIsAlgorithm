package ch12

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class FastExponentiationTest {
    @Test fun smallPowers() {
        assertEquals(2L,    power(2, 1))
        assertEquals(1024L, power(2, 10))
        assertEquals(81L,   power(3, 4))
    }
    @Test fun baseZeroReturnsOne() = assertEquals(1L, power(0, 5))
    @Test fun matchesMainProgram() = assertEquals(1073741824L, power(2, 30))
}

class FibonacciDnCTest {
    @Test fun smallFibonacciValues() {
        assertEquals(1L,  fibonacciDnC(1))
        assertEquals(1L,  fibonacciDnC(2))
        assertEquals(2L,  fibonacciDnC(3))
        assertEquals(55L, fibonacciDnC(10))
    }
    @Test fun matchesMainProgram() = assertEquals(1836311903L, fibonacciDnC(46))
}

class MergeSortTest {
    @Test fun sortsMainProgramData() {
        val data = intArrayOf(334, 6, 4, 2, 3, 1, 5, 117, 12, 34)
        mergeSort(data, 0, data.lastIndex)
        assertEquals(intArrayOf(1, 2, 3, 4, 5, 6, 12, 34, 117, 334).toList(), data.toList())
    }
    @Test fun alreadySortedStaysSorted() {
        val data = intArrayOf(1, 2, 3, 4, 5)
        mergeSort(data, 0, data.lastIndex)
        assertEquals(intArrayOf(1, 2, 3, 4, 5).toList(), data.toList())
    }
    @Test fun singleElementUnchanged() {
        val data = intArrayOf(42)
        mergeSort(data, 0, 0)
        assertEquals(listOf(42), data.toList())
    }
}
