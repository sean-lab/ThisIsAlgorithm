package ch06

/**
 * 이진 탐색 트리 (Binary Search Tree)
 * Rust/ch06/src/binary_search_tree.rs → Kotlin 이디엄 포팅
 */
class BSTNode(var data: Int) {
    var left: BSTNode? = null
    var right: BSTNode? = null
}

class BinarySearchTree {
    var root: BSTNode? = null

    fun insert(data: Int) {
        val newNode = BSTNode(data)
        if (root == null) { root = newNode; return }
        insertHelper(root!!, newNode)
    }

    private fun insertHelper(tree: BSTNode, newNode: BSTNode) {
        if (tree.data < newNode.data) {
            if (tree.right == null) tree.right = newNode
            else insertHelper(tree.right!!, newNode)
        } else if (tree.data > newNode.data) {
            if (tree.left == null) tree.left = newNode
            else insertHelper(tree.left!!, newNode)
        }
    }

    fun search(target: Int): BSTNode? = searchHelper(root, target)

    private fun searchHelper(node: BSTNode?, target: Int): BSTNode? {
        if (node == null) return null
        return when {
            node.data == target -> node
            node.data > target -> searchHelper(node.left, target)
            else -> searchHelper(node.right, target)
        }
    }

    fun searchMin(node: BSTNode?): BSTNode? {
        if (node == null) return null
        return if (node.left == null) node else searchMin(node.left)
    }

    /** 대상 노드를 트리에서 제거하고 해당 노드를 반환한다. */
    fun remove(target: Int): BSTNode? = removeHelper(root, null, target)

    private fun removeHelper(node: BSTNode?, parent: BSTNode?, target: Int): BSTNode? {
        if (node == null) return null
        return when {
            node.data > target -> removeHelper(node.left, node, target)
            node.data < target -> removeHelper(node.right, node, target)
            else -> {
                val removed: BSTNode
                when {
                    node.left == null && node.right == null -> {
                        removed = node
                        when {
                            parent == null -> root = null
                            parent.left === node -> parent.left = null
                            else -> parent.right = null
                        }
                    }
                    node.left != null && node.right != null -> {
                        val minNode = searchMin(node.right)!!
                        val minRemoved = removeHelper(node, null, minNode.data)!!
                        node.data = minRemoved.data
                        removed = minRemoved
                    }
                    else -> {
                        removed = node
                        val child = node.left ?: node.right
                        when {
                            parent == null -> root = child
                            parent.left === node -> parent.left = child
                            else -> parent.right = child
                        }
                    }
                }
                removed
            }
        }
    }

    fun inorder(): List<Int> = buildList { inorderHelper(root, this) }

    private fun inorderHelper(node: BSTNode?, out: MutableList<Int>) {
        if (node == null) return
        inorderHelper(node.left, out)
        out.add(node.data)
        inorderHelper(node.right, out)
    }
}

fun main() {
    val bst = BinarySearchTree()
    bst.insert(123)
    listOf(22, 9918, 424, 17, 3, 98, 34, 760, 317, 1).forEach { bst.insert(it) }
    println("Inorder: ${bst.inorder()}")
    bst.remove(98)
    println("After remove 98: ${bst.inorder()}")
}
