package ch15

/**
 * 미로 찾기 (Maze Solver, 백트래킹)
 * Rust/ch15/src/maze_solver.rs → Kotlin 이디엄 포팅
 */
const val START:  Byte = 'S'.code.toByte()
const val GOAL:   Byte = 'G'.code.toByte()
const val WAY:    Byte = ' '.code.toByte()
const val WALL:   Byte = '#'.code.toByte()
const val MARKED: Byte = '+'.code.toByte()

data class Position(val x: Int, val y: Int)

class MazeInfo(val data: Array<ByteArray>) {
    val rowSize    get() = data.size
    val columnSize get() = data[0].size
}

fun solveMaze(maze: MazeInfo): Boolean {
    var start: Position? = null
    outer@ for (i in 0 until maze.rowSize) {
        for (j in 0 until maze.columnSize) {
            if (maze.data[i][j] == START) { start = Position(j, i); break@outer }
        }
    }
    val s = start ?: return false
    val found = moveTo(maze, s)
    maze.data[s.y][s.x] = START
    return found
}

private fun moveTo(maze: MazeInfo, cur: Position): Boolean {
    if (maze.data[cur.y][cur.x] == GOAL) return true
    maze.data[cur.y][cur.x] = MARKED
    for (dir in 0..3) {
        val next = getNextStep(maze, cur, dir) ?: continue
        if (moveTo(maze, next)) return true
    }
    maze.data[cur.y][cur.x] = WAY
    return false
}

private fun getNextStep(maze: MazeInfo, cur: Position, dir: Int): Position? {
    val next = when (dir) {
        0 -> if (cur.y - 1 < 0)              null else Position(cur.x, cur.y - 1)
        1 -> if (cur.y + 1 >= maze.rowSize)   null else Position(cur.x, cur.y + 1)
        2 -> if (cur.x + 1 >= maze.columnSize)null else Position(cur.x + 1, cur.y)
        3 -> if (cur.x - 1 < 0)              null else Position(cur.x - 1, cur.y)
        else -> null
    } ?: return null
    val cell = maze.data[next.y][next.x]
    if (cell == WALL || cell == MARKED) return null
    return next
}

fun main() {
    val rows = arrayOf("S G")
    val maze = MazeInfo(Array(rows.size) { rows[it].toByteArray(Charsets.ISO_8859_1) })
    println(if (solveMaze(maze)) "Solved!" else "No solution")
}
