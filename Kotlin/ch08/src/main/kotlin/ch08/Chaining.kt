package ch08

/**
 * 체이닝 해시 테이블 (Chaining)
 * Rust/ch08/src/chaining.rs → Kotlin 이디엄 포팅
 */
class ChainingHashTable(val tableSize: Int = 12289) {
    private data class Node(val key: String, val value: String, val next: Node?)

    private val table = arrayOfNulls<Node>(tableSize)

    companion object {
        fun hash(key: String, tableSize: Int): Int {
            var h = 0
            for (b in key.toByteArray()) {
                h = (h shl 3) + (b.toInt() and 0xFF)
            }
            return Math.floorMod(h, tableSize)
        }
    }

    fun set(key: String, value: String) {
        val addr = hash(key, tableSize)
        table[addr] = Node(key, value, table[addr])
    }

    fun get(key: String): String? {
        val addr = hash(key, tableSize)
        var node = table[addr]
        while (node != null) {
            if (node.key == key) return node.value
            node = node.next
        }
        return null
    }
}

fun main() {
    val ht = ChainingHashTable()
    ht.set("MSFT", "Microsoft Corporation")
    ht.set("APAC", "Apache Org")
    ht.set("ZYMZZ", "Unisys Ops Check")
    println(ht.get("MSFT"))
    println(ht.get("APAC"))
    println(ht.get("ZYMZZ"))
}
