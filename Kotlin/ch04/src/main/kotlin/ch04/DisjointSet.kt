package ch04

/**
 * 분리 집합 (Disjoint Set / Union-Find)
 * Rust/ch04/src/disjoint_set.rs → Kotlin 이디엄 포팅
 * 객체 동일성(===)으로 집합 대표 노드를 비교한다.
 */
class DisjointSet(val data: Int) {
    var parent: DisjointSet? = null

    /** 루트(대표 원소)를 찾는다. */
    fun findRoot(): DisjointSet {
        var set = this
        while (set.parent != null) set = set.parent!!
        return set
    }
}

/** set2의 루트를 set1에 연결한다 (합집합). */
fun dsUnion(set1: DisjointSet, set2: DisjointSet) {
    val root2 = set2.findRoot()
    root2.parent = set1
}

fun main() {
    val sets = Array(5) { DisjointSet(it + 1) }

    dsUnion(sets[0], sets[2])   // 1과 3을 합침
    dsUnion(sets[2], sets[3])   // 3과 4를 합침

    for (i in 0 until 5) {
        print("Set[${i + 1}] root: ${sets[i].findRoot().data}  ")
    }
    println()
}
