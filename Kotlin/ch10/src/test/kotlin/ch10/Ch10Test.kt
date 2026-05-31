package ch10

import kotlin.test.Test
import kotlin.test.assertEquals

// 공통 테스트 케이스: 4가지 알고리즘이 동일한 결과를 반환해야 한다.
private val CASES = listOf(
    Triple("hello world", "world", 6),
    Triple("abcabcabd", "abcabd", 3),
    Triple("aaaaa", "aaa", 0),
    Triple("hello", "xyz", -1)
)

class BruteForceTest {
    @Test fun findsPatternPosition() = CASES.forEach { (t, p, exp) -> assertEquals(exp, bruteForce(t, p)) }
}

class KarpRabinTest {
    @Test fun findsPatternPosition() = CASES.forEach { (t, p, exp) -> assertEquals(exp, karpRabin(t, p)) }

    @Test
    fun equalStringsHaveEqualHash() {
        val a = "abcd".toByteArray()
        assertEquals(krHash(a, 4), krHash("abcd".toByteArray(), 4))
    }
}

class KnuthMorrisPrattTest {
    @Test fun findsPatternPosition() = CASES.forEach { (t, p, exp) -> assertEquals(exp, knuthMorrisPratt(t, p)) }

    @Test
    fun preprocessBorderForAbabaca() {
        val border = kmpPreprocess("ababaca".toByteArray())
        assertEquals(listOf(-1, 0, 0, 1, 2, 3, 0, 1), border.toList())
    }
}

class BoyerMooreTest {
    @Test fun findsPatternPosition() = CASES.forEach { (t, p, exp) -> assertEquals(exp, boyerMoore(t, p)) }

    @Test
    fun badCharTableRecordsLastIndex() {
        val bct = buildBadCharTable("abcab".toByteArray())
        assertEquals(3, bct['a'.code])
        assertEquals(4, bct['b'.code])
        assertEquals(2, bct['c'.code])
        assertEquals(-1, bct['z'.code])
    }
}
