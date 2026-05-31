package ch09

/**
 * 다익스트라 최단 경로 (Dijkstra)
 * Rust/ch09/src/bin/dijkstra.rs → Kotlin 이디엄 포팅
 */

const val MAX_WEIGHT = 36267

data class ShortestPath(val fromIndex: Int, val toIndex: Int, val totalWeight: Int)

fun dijkstra(g: Graph, start: Vertex): List<ShortestPath> {
    val n = g.vertexCount
    val weights = IntArray(n) { MAX_WEIGHT }
    val fringes = BooleanArray(n)
    val precedences = arrayOfNulls<Vertex>(n)

    weights[start.index] = 0

    // PQ: (cumulative weight, vertex index) — 누적 거리 기반 우선순위
    val pq = java.util.PriorityQueue<Pair<Int, Vertex>>(compareBy { it.first })
    pq.add(0 to start)

    while (pq.isNotEmpty()) {
        val (_, current) = pq.poll()
        if (fringes[current.index]) continue
        fringes[current.index] = true

        for (e in current.adjacencyList) {
            val t = e.target
            val newDist = weights[current.index] + e.weight
            if (!fringes[t.index] && newDist < weights[t.index]) {
                weights[t.index] = newDist
                precedences[t.index] = current
                pq.add(newDist to t)
            }
        }
    }

    return (0 until n).mapNotNull { i ->
        val prec = precedences[i] ?: return@mapNotNull null
        ShortestPath(prec.index, i, weights[i])
    }
}

fun main() {
    val g = Graph()
    val a = Vertex('A'.code); val b = Vertex('B'.code); val c = Vertex('C'.code)
    val d = Vertex('D'.code); val e = Vertex('E'.code); val f = Vertex('F'.code)
    val gg = Vertex('G'.code); val h = Vertex('H'.code); val ii = Vertex('I'.code)
    listOf(a, b, c, d, e, f, gg, h, ii).forEach { g.addVertex(it) }

    g.addEdge(a, b, 35); g.addEdge(a, e, 247)
    g.addEdge(b, a, 35); g.addEdge(b, c, 126); g.addEdge(b, f, 150)
    g.addEdge(c, b, 126); g.addEdge(c, d, 117); g.addEdge(c, f, 162); g.addEdge(c, gg, 220)
    g.addEdge(d, c, 117)
    g.addEdge(e, a, 247); g.addEdge(e, f, 82); g.addEdge(e, h, 98)
    g.addEdge(f, b, 150); g.addEdge(f, c, 162); g.addEdge(f, e, 82); g.addEdge(f, gg, 154); g.addEdge(f, h, 120)
    g.addEdge(gg, c, 220); g.addEdge(gg, f, 154); g.addEdge(gg, ii, 106)
    g.addEdge(h, e, 98); g.addEdge(h, f, 120)
    g.addEdge(ii, gg, 106)

    val paths = dijkstra(g, b)
    for (p in paths) {
        println("${g.vertices[p.fromIndex].data.toChar()} → ${g.vertices[p.toIndex].data.toChar()} : ${p.totalWeight}")
    }
}
