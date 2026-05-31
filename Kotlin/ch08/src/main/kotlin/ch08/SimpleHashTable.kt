package ch08

/**
 * 단순 해시 테이블 (Simple Hash Table) — 직접 주소화
 * Rust/ch08/src/simple_hash_table.rs → Kotlin 이디엄 포팅
 */
class SimpleHashTable(val tableSize: Int) {
    private val keys = IntArray(tableSize)
    private val values = IntArray(tableSize)

    fun hash(key: Int): Int = key % tableSize

    fun set(key: Int, value: Int) {
        val addr = hash(key)
        keys[addr] = key
        values[addr] = value
    }

    fun get(key: Int): Int = values[hash(key)]
}

fun main() {
    val ht = SimpleHashTable(193)
    ht.set(418, 32114); ht.set(9, 514); ht.set(27, 8917); ht.set(1031, 286)
    println(ht.get(418)); println(ht.get(9)); println(ht.get(27)); println(ht.get(1031))
}
