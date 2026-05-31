package ch14

import java.util.PriorityQueue

/**
 * 허프만 코딩 (Huffman Coding)
 * Rust/ch14/src/huffman.rs → Kotlin 이디엄 포팅
 */

private sealed class HuffmanNode(val freq: Int) {
    class Leaf(val symbol: Byte, freq: Int) : HuffmanNode(freq)
    class Internal(val left: HuffmanNode, val right: HuffmanNode, freq: Int) : HuffmanNode(freq)
}

private fun buildTree(source: ByteArray): HuffmanNode {
    val freq = IntArray(256)
    for (b in source) freq[b.toInt() and 0xFF]++
    val pq: PriorityQueue<HuffmanNode> = PriorityQueue(compareBy { it.freq })
    for (i in 0..255) {
        if (freq[i] > 0) pq.add(HuffmanNode.Leaf(i.toByte(), freq[i]))
    }
    while (pq.size > 1) {
        val left = pq.poll()
        val right = pq.poll()
        pq.add(HuffmanNode.Internal(left, right, left.freq + right.freq))
    }
    return pq.poll()
}

private fun buildCodeTable(node: HuffmanNode, prefix: BooleanArray, depth: Int, table: Array<BooleanArray?>) {
    when (node) {
        is HuffmanNode.Leaf -> table[node.symbol.toInt() and 0xFF] = prefix.copyOf(depth)
        is HuffmanNode.Internal -> {
            prefix[depth] = false
            buildCodeTable(node.left, prefix, depth + 1, table)
            prefix[depth] = true
            buildCodeTable(node.right, prefix, depth + 1, table)
        }
    }
}

private class BitBuffer {
    private val bytes = mutableListOf<Byte>()
    var size = 0
    fun addBit(bit: Boolean) {
        if (size % 8 == 0) bytes.add(0)
        val mask = (0x80 ushr (size % 8)).toByte()
        val idx = size / 8
        if (bit) bytes[idx] = (bytes[idx].toInt() or mask.toInt()).toByte()
        size++
    }
    fun toBinaryString(): String {
        val sb = StringBuilder(size)
        for (i in 0 until size) {
            val mask = 0x80 ushr (i % 8)
            sb.append(if ((bytes[i / 8].toInt() and 0xFF and mask) != 0) '1' else '0')
        }
        return sb.toString()
    }
    fun decodeWith(root: HuffmanNode): ByteArray {
        val result = mutableListOf<Byte>()
        var current: HuffmanNode = root
        for (i in 0..size) {
            if (current is HuffmanNode.Leaf) {
                result.add(current.symbol)
                current = root
            }
            if (i == size) break
            val mask = 0x80 ushr (i % 8)
            current = if ((bytes[i / 8].toInt() and 0xFF and mask) == 0)
                (current as HuffmanNode.Internal).left
            else
                (current as HuffmanNode.Internal).right
        }
        return result.toByteArray()
    }
}

data class HuffmanResult(
    val originalSize: Int,
    val encodedSize: Int,
    val original: String,
    val binary: String,
    val decoded: String
)

fun huffmanRun(source: String): HuffmanResult {
    val bytes = source.toByteArray(Charsets.ISO_8859_1)
    val tree = buildTree(bytes)
    val table: Array<BooleanArray?> = arrayOfNulls(256)
    buildCodeTable(tree, BooleanArray(256), 0, table)
    val buf = BitBuffer()
    for (b in bytes) {
        val code = table[b.toInt() and 0xFF]!!
        for (bit in code) buf.addBit(bit)
    }
    val decoded = buf.decodeWith(tree).toString(Charsets.ISO_8859_1)
    return HuffmanResult(
        originalSize = (source.length + 1) * 8,
        encodedSize = buf.size,
        original = source,
        binary = buf.toBinaryString(),
        decoded = decoded
    )
}

fun main() {
    val r = huffmanRun("This Is Algorithms.")
    println("original bits: ${r.originalSize}, encoded bits: ${r.encodedSize}")
    println("decoded: ${r.decoded}")
}
