package ch09

/**
 * 그래프 (Graph) - 인접 리스트 기반
 * Rust/ch09/src/graph.rs → Kotlin 이디엄 포팅 (안전한 참조 사용)
 */
class Vertex(val data: Int) {
    var visited: Boolean = false
    var index: Int = -1
    val adjacencyList: MutableList<Edge> = mutableListOf()
}

class Edge(val from: Vertex, val target: Vertex, val weight: Int)

class Graph {
    private val _vertices: MutableList<Vertex> = mutableListOf()
    val vertices: List<Vertex> get() = _vertices
    val vertexCount: Int get() = _vertices.size

    fun addVertex(v: Vertex) {
        v.index = _vertices.size
        _vertices.add(v)
    }

    fun addEdge(from: Vertex, target: Vertex, weight: Int = 0) {
        from.adjacencyList.add(Edge(from, target, weight))
    }

    fun resetVisited() = _vertices.forEach { it.visited = false }

    fun printGraph() {
        for (v in _vertices) {
            print("${v.data.toChar()} : ")
            for (e in v.adjacencyList) print("${e.target.data.toChar()}[${e.weight}] ")
            println()
        }
    }
}

fun main() {
    val g = Graph()
    val a = Vertex('A'.code); val b = Vertex('B'.code); val c = Vertex('C'.code)
    g.addVertex(a); g.addVertex(b); g.addVertex(c)
    g.addEdge(a, b, 5); g.addEdge(a, c, 7)
    g.printGraph()
}
