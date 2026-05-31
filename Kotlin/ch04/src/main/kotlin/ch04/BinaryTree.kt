package ch04

/**
 * 이진 트리 (Binary Tree)
 * Rust/ch04/src/binary_tree.rs → Kotlin 이디엄 포팅
 */
class BinaryNode(val data: Char) {
    var left: BinaryNode? = null
    var right: BinaryNode? = null
}

fun preorder(node: BinaryNode?, out: MutableList<Char> = mutableListOf()): List<Char> {
    if (node != null) {
        out.add(node.data)
        preorder(node.left, out)
        preorder(node.right, out)
    }
    return out
}

fun inorder(node: BinaryNode?, out: MutableList<Char> = mutableListOf()): List<Char> {
    if (node != null) {
        inorder(node.left, out)
        out.add(node.data)
        inorder(node.right, out)
    }
    return out
}

fun postorder(node: BinaryNode?, out: MutableList<Char> = mutableListOf()): List<Char> {
    if (node != null) {
        postorder(node.left, out)
        postorder(node.right, out)
        out.add(node.data)
    }
    return out
}

fun main() {
    //        A
    //      /   \
    //     B     E
    //    / \   / \
    //   C   D F   G
    val b = BinaryNode('B').also {
        it.left = BinaryNode('C')
        it.right = BinaryNode('D')
    }
    val e = BinaryNode('E').also {
        it.left = BinaryNode('F')
        it.right = BinaryNode('G')
    }
    val a = BinaryNode('A').also {
        it.left = b
        it.right = e
    }

    println("Preorder:  ${preorder(a)}")
    println("Inorder:   ${inorder(a)}")
    println("Postorder: ${postorder(a)}")
}
