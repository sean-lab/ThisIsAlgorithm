package ch15

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

private fun mazeFrom(rows: Array<String>): MazeInfo =
    MazeInfo(Array(rows.size) { rows[it].toByteArray(Charsets.ISO_8859_1) })

class MazeSolverTest {
    @Test fun solvesSimpleMaze() {
        val maze = mazeFrom(arrayOf("S G"))
        assertTrue(solveMaze(maze))
        assertEquals(START, maze.data[0][0])   // start 복원
        assertEquals(MARKED, maze.data[0][1])  // 통로에 표시 남음
    }
    @Test fun unsolvableWhenWalledOff() {
        val maze = mazeFrom(arrayOf("S#G"))
        assertFalse(solveMaze(maze))
    }
    @Test fun failsWithoutStart() {
        val maze = mazeFrom(arrayOf("  G"))
        assertFalse(solveMaze(maze))
    }
}

class NQueensTest {
    @Test fun isThreatened_detectsSameColumn() {
        val cols = intArrayOf(3, 1, 3)
        assertTrue(isThreatened(cols, 2))
    }
    @Test fun isThreatened_detectsDiagonal() {
        val cols = intArrayOf(0, 1)
        assertTrue(isThreatened(cols, 1))
    }
    @Test fun knownSolutionCounts() {
        assertEquals(2,  countSolutions(4))
        assertEquals(4,  countSolutions(6))
        assertEquals(92, countSolutions(8))
    }
}
