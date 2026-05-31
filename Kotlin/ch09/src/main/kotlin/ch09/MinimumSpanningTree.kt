package ch09

/**
 * 최소 신장 트리 (Prim / Kruskal)
 * Rust/ch09/src/bin/mst.rs → Kotlin 이디엄 포팅
 */

data class MSTEdge(val fromIndex: Int, val toIndex: Int, val weight: Int)

// Prim 알고리즘 (우선순위 큐 기반)
fun prim(g: Graph, start: Vertex): List<MSTEdge> {
    val n = g.vertexCount
    val weights = IntArray(n) { MAX_WEIGHT }
    val fringes = BooleanArray(n)
    val precedences = arrayOfNulls<Vertex>(n)

    weights[start.index] = 0

    val pq = java.util.PriorityQueue<Pair<Int, Vertex>>(compareBy { it.first })
    pq.add(0 to start)

    while (pq.isNotEmpty()) {
        val (_, current) = pq.poll()
        if (fringes[current.index]) continue
        fringes[current.index] = true

        for (e in current.adjacencyList) {
            val t = e.target
            if (!fringes[t.index] && e.weight < weights[t.index]) {
                weights[t.index] = e.weight
                precedences[t.index] = current
                pq.add(e.weight to t)
            }
        }
    }

    return (0 until n).mapNotNull { i ->
        val prec = precedences[i] ?: return@mapNotNull null
        MSTEdge(prec.index, i, weights[i])
    }
}

// Union-Find (크루스칼용)
private class UnionFind(n: Int) {
    private val parent = IntArray(n) { it }
    private val rank = IntArray(n)

    fun find(x: Int): Int {
        if (parent[x] != x) parent[x] = find(parent[x])
        return parent[x]
    }

    fun union(x: Int, y: Int): Boolean {
        val px = find(x); val py = find(y)
        if (px == py) return false
        when {
            rank[px] < rank[py] -> parent[px] = py
            rank[px] > rank[py] -> parent[py] = px
            else -> { parent[py] = px; rank[px]++ }
        }
        return true
    }
}

// Kruskal 알고리즘
fun kruskal(g: Graph): List<MSTEdge> {
    // 무방향 그래프에서 중복 엣지 제거 (from.index < target.index 인 것만)
    val edges = mutableListOf<Triple<Int, Int, Int>>()
    for (v in g.vertices) {
        for (e in v.adjacencyList) {
            if (v.index < e.target.index) edges.add(Triple(e.weight, v.index, e.target.index))
        }
    }
    edges.sortBy { it.first }

    val uf = UnionFind(g.vertexCount)
    val result = mutableListOf<MSTEdge>()
    for ((w, from, to) in edges) {
        if (uf.union(from, to)) result.add(MSTEdge(from, to, w))
    }
    return result
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

    println("Prim:")
    prim(g, b).forEach { println("  ${g.vertices[it.fromIndex].data.toChar()}-${g.vertices[it.toIndex].data.toChar()}:${it.weight}") }
    println("Kruskal:")
    kruskal(g).forEach { println("  ${g.vertices[it.fromIndex].data.toChar()}-${g.vertices[it.toIndex].data.toChar()}:${it.weight}") }
}
