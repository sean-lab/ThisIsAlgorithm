package ch04

/**
 * LCRS 트리 (Left-Child Right-Sibling Tree)
 * Rust/ch04/src/lcrs_tree.rs → Kotlin 이디엄 포팅
 */
class LCRSNode(val data: Char) {
    var leftChild: LCRSNode? = null
    var rightSibling: LCRSNode? = null
}

fun addChildNode(parent: LCRSNode, child: LCRSNode) {
    if (parent.leftChild == null) {
        parent.leftChild = child
    } else {
        var temp = parent.leftChild!!
        while (temp.rightSibling != null) temp = temp.rightSibling!!
        temp.rightSibling = child
    }
}

fun printLCRSTree(node: LCRSNode, depth: Int) {
    repeat(depth - 1) { print("   ") }
    if (depth > 0) print("+--")
    println(node.data)
    node.leftChild?.let { printLCRSTree(it, depth + 1) }
    node.rightSibling?.let { printLCRSTree(it, depth) }
}

fun main() {
    val root = LCRSNode('A')
    val b = LCRSNode('B'); val c = LCRSNode('C'); val d = LCRSNode('D')
    val e = LCRSNode('E'); val f = LCRSNode('F'); val g = LCRSNode('G')
    val h = LCRSNode('H'); val i = LCRSNode('I')
    addChildNode(root, b); addChildNode(root, g); addChildNode(root, i)
    addChildNode(b, c); addChildNode(b, d); addChildNode(b, e); addChildNode(b, f)
    addChildNode(i, h)
    printLCRSTree(root, 0)
}
