package ch11

import kotlin.test.Test
import kotlin.test.assertEquals

class RecurrenceSumTest {
    @Test fun singleElementReturnsItself() = assertEquals(42, recurrenceSum(intArrayOf(42)))
    @Test fun sumsAllElements() = assertEquals(15, recurrenceSum(intArrayOf(1, 2, 3, 4, 5)))
    @Test fun sumOf1To55() {
        val data = IntArray(55) { it + 1 }
        assertEquals(1540, recurrenceSum(data))
    }
}
