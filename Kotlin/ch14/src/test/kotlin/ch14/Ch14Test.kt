package ch14

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class HuffmanTest {
    @Test fun roundtripRecoversOriginal() {
        val r = huffmanRun("This Is Algorithms.")
        assertEquals("This Is Algorithms.", r.decoded)
        assertEquals("This Is Algorithms.", r.original)
    }
    @Test fun encodedIsSmallerThanOriginal() {
        val r = huffmanRun("This Is Algorithms.")
        assertEquals(20 * 8, r.originalSize)
        assertTrue(r.encodedSize < r.originalSize)
    }
    @Test fun binaryLengthEqualsEncodedSize() {
        val r = huffmanRun("This Is Algorithms.")
        assertEquals(r.encodedSize, r.binary.length)
    }
}

class MakingChangeTest {
    @Test fun countCoinsDivides() {
        assertEquals(2, countCoins(2600, 1000))
        assertEquals(1, countCoins(600, 500))
        assertEquals(0, countCoins(100, 500))
    }
    @Test fun getChangeBreaksDownGreedily() {
        val units = intArrayOf(1000, 500, 100, 50, 10)
        val change = IntArray(5)
        getChange(7400, 10000, units, change)
        assertEquals(intArrayOf(2, 1, 1, 0, 0).toList(), change.toList())
    }
    @Test fun exactPaymentYieldsNoChange() {
        val units = intArrayOf(1000, 500, 100)
        val change = IntArray(3)
        getChange(1500, 1500, units, change)
        assertEquals(intArrayOf(0, 0, 0).toList(), change.toList())
    }
}
