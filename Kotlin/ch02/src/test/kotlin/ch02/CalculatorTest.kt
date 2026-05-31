package ch02

import kotlin.test.*

class CalculatorTest {

    @Test
    fun `postfix simple`() {
        assertEquals("3 4 +", getPostfix("3+4"))
    }

    @Test
    fun `postfix precedence`() {
        assertEquals("3 4 2 *+", getPostfix("3+4*2"))
    }

    @Test
    fun `postfix parentheses`() {
        assertEquals("3 4 +2 *", getPostfix("(3+4)*2"))
    }

    @Test
    fun `calculate basic`() {
        assertEquals(7.0, calculate(getPostfix("3+4")))
        assertEquals(11.0, calculate(getPostfix("3+4*2")))
        assertEquals(14.0, calculate(getPostfix("(3+4)*2")))
    }

    @Test
    fun `calculate division`() {
        assertEquals(2.5, calculate(getPostfix("10/4")))
    }

    @Test
    fun `gcvt10 trims trailing zeros`() {
        assertEquals("7", gcvt10(7.0))
        assertEquals("2.5", gcvt10(2.5))
        assertEquals("0", gcvt10(0.0))
    }

    @Test
    fun `priority rules`() {
        // 스택의 + (우선순위 숫자 2) > 입력의 * (우선순위 숫자 1) → isPrior=true → 팝하지 않음
        assertTrue(isPrior('+', '*'))
        assertFalse(isPrior('*', '+'))
    }
}
