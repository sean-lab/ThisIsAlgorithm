package ch13

import kotlin.test.Test
import kotlin.test.assertEquals

class FibonacciDPTest {
    @Test fun baseCases() {
        assertEquals(0L, fibonacciDP(0))
        assertEquals(1L, fibonacciDP(1))
    }
    @Test fun smallValues() {
        assertEquals(1L,  fibonacciDP(2))
        assertEquals(55L, fibonacciDP(10))
    }
    @Test fun matchesMainProgram() = assertEquals(1836311903L, fibonacciDP(46))
}

class LCSDCTest {
    @Test fun lcsLengthOfMainStrings() {
        val x = "GOOD MORNING.".toByteArray()
        val y = "GUTEN MORGEN.".toByteArray()
        val table = Array(x.size + 1) { IntArray(y.size + 1) }
        assertEquals(7, lcsdc(x, y, x.size, y.size, table))
    }
    @Test fun simpleCases() {
        val abc = "ABC".toByteArray()
        var t = Array(4) { IntArray(4) }
        assertEquals(3, lcsdc(abc, abc, 3, 3, t))
        t = Array(4) { IntArray(4) }
        assertEquals(2, lcsdc(abc, "AXC".toByteArray(), 3, 3, t))
        t = Array(4) { IntArray(4) }
        assertEquals(0, lcsdc(abc, "XYZ".toByteArray(), 3, 3, t))
    }
}

class LCSDPTest {
    private fun run(xs: String, ys: String): Pair<Int, String> {
        val x = xs.toByteArray()
        val y = ys.toByteArray()
        val table = Array(x.size + 1) { IntArray(y.size + 1) }
        val len = lcsdp(x, y, table)
        val sb = StringBuilder()
        lcsTraceBack(x, y, x.size, y.size, table, sb)
        return len to sb.toString()
    }
    @Test fun mainProgramLcsAndTraceback() {
        val (len, lcs) = run("GOOD MORNING.", "GUTEN MORGEN.")
        assertEquals(7, len)
        assertEquals("G MORN.", lcs)
    }
    @Test fun identicalStrings() {
        val (len, lcs) = run("ABC", "ABC")
        assertEquals(3, len)
        assertEquals("ABC", lcs)
    }
    @Test fun noCommonSubsequence() {
        val (len, lcs) = run("ABC", "XYZ")
        assertEquals(0, len)
        assertEquals("", lcs)
    }
}
