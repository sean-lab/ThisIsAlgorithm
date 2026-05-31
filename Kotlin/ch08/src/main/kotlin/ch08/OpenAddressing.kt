package ch08

/**
 * 개방 주소법 해시 테이블 (Open Addressing, 이중 해싱)
 * Rust/ch08/src/open_addressing.rs → Kotlin 이디엄 포팅
 */
class OpenAddressHashTable(initialSize: Int = 11) {
    private enum class Status { EMPTY, OCCUPIED }
    private data class Element(val key: String, val value: String, val status: Status)

    var tableSize: Int = initialSize
        private set
    private var table = arrayOfNulls<Element>(initialSize)
    var occupiedCount: Int = 0
        private set

    companion object {
        fun hash(key: String, tableSize: Int): Int {
            var h = 0
            for (b in key.toByteArray()) h = (h shl 3) + (b.toInt() and 0xFF)
            return Math.floorMod(h, tableSize)
        }

        fun hash2(key: String, tableSize: Int): Int {
            var h = 0
            for (b in key.toByteArray()) h = (h shl 2) + (b.toInt() and 0xFF)
            return Math.floorMod(h, tableSize - 3) + 1
        }
    }

    fun set(key: String, value: String) {
        if (occupiedCount.toDouble() / tableSize > 0.5) rehash()

        var addr = hash(key, tableSize)
        val step = hash2(key, tableSize)

        while (table[addr] != null && table[addr]!!.status == Status.OCCUPIED && table[addr]!!.key != key) {
            addr = (addr + step) % tableSize
        }

        table[addr] = Element(key, value, Status.OCCUPIED)
        occupiedCount++
    }

    fun get(key: String): String? {
        var addr = hash(key, tableSize)
        val step = hash2(key, tableSize)

        while (table[addr] != null && table[addr]!!.status == Status.OCCUPIED && table[addr]!!.key != key) {
            addr = (addr + step) % tableSize
        }

        return if (table[addr]?.status == Status.OCCUPIED) table[addr]!!.value else null
    }

    private fun rehash() {
        val newSize = tableSize * 2
        val old = table.filterNotNull().filter { it.status == Status.OCCUPIED }
        tableSize = newSize
        table = arrayOfNulls(newSize)
        occupiedCount = 0
        for (e in old) set(e.key, e.value)
    }
}

fun main() {
    val ht = OpenAddressHashTable(11)
    listOf(
        "MSFT" to "Microsoft Corporation", "JAVA" to "Sun Microsystems",
        "REDH" to "Red Hat Linux", "APAC" to "Apache Org",
        "ZYMZZ" to "Unisys Ops Check", "IBM" to "IBM Ltd.",
        "ORCL" to "Oracle Corporation", "CSCO" to "Cisco Systems, Inc."
    ).forEach { (k, v) -> ht.set(k, v) }
    println(ht.get("MSFT"))
    println(ht.get("APAC"))
    println(ht.get("CSCO"))
}
