package ch09

/**
 * 그래프 순회 (DFS / BFS)
 * Rust/ch09/src/bin/graph_traversal.rs → Kotlin 이디엄 포팅
 */

fun dfs(start: Vertex): List<Int> {
    val result = mutableListOf<Int>()
    fun visit(v: Vertex) {
        result.add(v.data)
        v.visited = true
        for (e in v.adjacencyList) {
            if (!e.target.visited) visit(e.target)
        }
    }
    visit(start)
    return result
}

fun bfs(start: Vertex): List<Int> {
    val result = mutableListOf<Int>()
    val queue = ArrayDeque<Vertex>()
    start.visited = true
    result.add(start.data)
    queue.add(start)
    while (queue.isNotEmpty()) {
        val v = queue.removeFirst()
        for (e in v.adjacencyList) {
            if (!e.target.visited) {
                e.target.visited = true
                result.add(e.target.data)
                queue.add(e.target)
            }
        }
    }
    return result
}

fun main() {
    val g = Graph()
    val vs = (1..7).map { Vertex(it).also { v -> g.addVertex(v) } }
    g.addEdge(vs[0], vs[1]); g.addEdge(vs[0], vs[2])
    g.addEdge(vs[1], vs[3]); g.addEdge(vs[1], vs[4])
    g.addEdge(vs[2], vs[3]); g.addEdge(vs[2], vs[5])
    g.addEdge(vs[3], vs[4]); g.addEdge(vs[3], vs[6])
    g.addEdge(vs[4], vs[6])
    g.addEdge(vs[5], vs[6])

    println("DFS: ${dfs(vs[0])}")
    g.resetVisited()
    println("BFS: ${bfs(vs[0])}")
}
