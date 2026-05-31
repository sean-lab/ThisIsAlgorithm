package ch09

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class GraphTest {
    private fun buildSimpleGraph(): Triple<Graph, Vertex, Vertex> {
        val g = Graph()
        val a = Vertex('A'.code); val b = Vertex('B'.code); val c = Vertex('C'.code)
        g.addVertex(a); g.addVertex(b); g.addVertex(c)
        return Triple(g, a, b)
    }

    @Test
    fun addVertexAssignsIndexAndCounts() {
        val g = Graph()
        val a = Vertex('A'.code); val b = Vertex('B'.code); val c = Vertex('C'.code)
        g.addVertex(a); g.addVertex(b); g.addVertex(c)
        assertEquals(3, g.vertexCount)
        assertEquals(0, a.index); assertEquals(1, b.index); assertEquals(2, c.index)
    }

    @Test
    fun addEdgeBuildsAdjacencyList() {
        val g = Graph()
        val a = Vertex('A'.code); val b = Vertex('B'.code); val c = Vertex('C'.code)
        g.addVertex(a); g.addVertex(b); g.addVertex(c)
        g.addEdge(a, b, 5); g.addEdge(a, c, 7)
        assertEquals(2, a.adjacencyList.size)
        assertEquals('B'.code, a.adjacencyList[0].target.data)
        assertEquals(5, a.adjacencyList[0].weight)
        assertEquals('C'.code, a.adjacencyList[1].target.data)
        assertEquals(7, a.adjacencyList[1].weight)
    }
}

// 그래프 탐색 테스트용 그래프: 1→2,3 / 2→4,5 / 3→4,6 / 4→5,7 / 5→7 / 6→7
private fun buildTraversalGraph(): Pair<Graph, List<Vertex>> {
    val g = Graph()
    val vs = (1..7).map { Vertex(it) }
    vs.forEach { g.addVertex(it) }
    g.addEdge(vs[0], vs[1]); g.addEdge(vs[0], vs[2])
    g.addEdge(vs[1], vs[3]); g.addEdge(vs[1], vs[4])
    g.addEdge(vs[2], vs[3]); g.addEdge(vs[2], vs[5])
    g.addEdge(vs[3], vs[4]); g.addEdge(vs[3], vs[6])
    g.addEdge(vs[4], vs[6])
    g.addEdge(vs[5], vs[6])
    return g to vs
}

class GraphTraversalTest {
    @Test
    fun dfsVisitsAllVerticesInExpectedOrder() {
        val (g, vs) = buildTraversalGraph()
        val result = dfs(vs[0])
        assertEquals(listOf(1, 2, 4, 5, 7, 3, 6), result)
    }

    @Test
    fun bfsVisitsAllVerticesInExpectedOrder() {
        val (g, vs) = buildTraversalGraph()
        val result = bfs(vs[0])
        assertEquals(listOf(1, 2, 3, 4, 5, 6, 7), result)
    }
}

// 다익스트라 / MST 테스트용 그래프 (A-B-C-D-E-F-G-H-I 무방향 가중치)
private fun buildWeightedGraph(): Pair<Graph, List<Vertex>> {
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
    return g to listOf(a, b, c, d, e, f, gg, h, ii)
}

class DijkstraTest {
    @Test
    fun shortestPathFromBIsCorrect() {
        val (g, vs) = buildWeightedGraph()
        val b = vs[1]
        val paths = dijkstra(g, b)
        val weightMap = paths.associate { it.toIndex to it.totalWeight }
        // 인덱스: A=0 B=1 C=2 D=3 E=4 F=5 G=6 H=7 I=8
        assertEquals(35, weightMap[0])    // B→A: 35
        assertEquals(126, weightMap[2])   // B→C: 126
        assertEquals(243, weightMap[3])   // B→C→D: 243
        assertEquals(232, weightMap[4])   // B→F→E: 232
        assertEquals(150, weightMap[5])   // B→F: 150
        assertEquals(304, weightMap[6])   // B→F→G: 304
        assertEquals(270, weightMap[7])   // B→F→H: 270
        assertEquals(410, weightMap[8])   // B→F→G→I: 410
    }
}

class MinimumSpanningTreeTest {
    @Test
    fun primMSTTotalWeightIsCorrect() {
        val (g, vs) = buildWeightedGraph()
        val edges = prim(g, vs[1]) // start from B
        val totalWeight = edges.sumOf { it.weight }
        assertEquals(8, edges.size)
        assertEquals(868, totalWeight)
    }

    @Test
    fun kruskalMSTTotalWeightIsCorrect() {
        val (g, _) = buildWeightedGraph()
        val edges = kruskal(g)
        val totalWeight = edges.sumOf { it.weight }
        assertEquals(8, edges.size)
        assertEquals(868, totalWeight)
    }
}

class TopologicalSortTest {
    @Test
    fun sortedOrderSatisfiesAllEdgeConstraints() {
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
        assertEquals(8, sorted.size)

        // 각 엣지에 대해 from이 to보다 먼저 등장해야 한다
        fun pos(data: Int) = sorted.indexOf(data)
        val edges = listOf(
            a to c, a to d, b to c, b to e,
            c to f, d to f, d to gg, e to gg, f to h, gg to h
        )
        for ((from, to) in edges) {
            assertTrue(pos(from.data) < pos(to.data),
                "Expected ${from.data.toChar()} before ${to.data.toChar()}")
        }
    }
}
