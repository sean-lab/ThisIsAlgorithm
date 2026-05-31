package ch09

/**
 * 위상 정렬 (Topological Sort) - DFS 역후위 순서
 * Rust/ch09/src/bin/topological_sort.rs → Kotlin 이디엄 포팅
 */

fun topologicalSort(g: Graph): List<Int> {
    val result = ArrayDeque<Int>()

    fun dfs(v: Vertex) {
        v.visited = true
        for (e in v.adjacencyList) {
            if (!e.target.visited) dfs(e.target)
        }
        result.addFirst(v.data)
    }

    for (v in g.vertices) {
        if (!v.visited) dfs(v)
    }

    return result.toList()
}

fun main() {
    val g = Graph()
    val a = Vertex('A'.code); val b = Vertex('B'.code); val c = Vertex('C'.code)
    val d = Vertex('D'.code); val e = Vertex('E'.code); val f = Vertex('F'.code)
    val gg = Vertex('G'.code); val h = Vertex('H'.code)
    listOf(a, b, c, d, e, f, gg, h).forEach { g.addVertex(it) }

    g.addEdge(a, c); g.addEdge(a, d)
    g.addEdge(b, c); g.addEdge(b, e)
    g.addEdge(c, f)
    g.addEdge(d, f); g.addEdge(d, gg)
    g.addEdge(e, gg)
    g.addEdge(f, h)
    g.addEdge(gg, h)

    val sorted = topologicalSort(g)
    println("Topological Sort Result : ${sorted.map { it.toChar() }.joinToString(" ")}")
}
