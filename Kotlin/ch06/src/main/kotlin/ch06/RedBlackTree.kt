package ch06

/**
 * 레드 블랙 트리 (Red-Black Tree)
 * Rust/ch06/src/red_black_tree.rs → Kotlin 이디엄 포팅
 * NIL 센티넬을 파일 수준 val 로 두고 object identity(===)로 비교한다.
 */
const val RBT_RED = 0
const val RBT_BLACK = 1

class RBTNode(var data: Int) {
    var parent: RBTNode? = null
    var left: RBTNode? = null
    var right: RBTNode? = null
    var color: Int = RBT_BLACK
}

/** 전역 Nil 센티넬 — C 의 `RBTNode* Nil; Nil->Color = BLACK;` 에 대응. */
val RBT_NIL = RBTNode(0).also { it.color = RBT_BLACK }

class RedBlackTree {
    var root: RBTNode? = null

    // ─── 공개 API ──────────────────────────────────────────────────────────────

    fun insert(newNode: RBTNode) {
        insertHelper(newNode)
        newNode.color = RBT_RED
        newNode.left = RBT_NIL
        newNode.right = RBT_NIL
        rebuildAfterInsert(newNode)
    }

    fun search(target: Int): RBTNode? = searchHelper(root, target)

    fun searchMin(node: RBTNode?): RBTNode? {
        if (node == null || node === RBT_NIL) return null
        return if (node.left === RBT_NIL) node else searchMin(node.left)
    }

    fun remove(data: Int): RBTNode? {
        val target = search(data) ?: return null

        val removed: RBTNode = if (target.left === RBT_NIL || target.right === RBT_NIL) {
            target
        } else {
            val minNode = searchMin(target.right)!!
            target.data = minNode.data
            minNode
        }

        val successor: RBTNode = if (removed.left !== RBT_NIL) removed.left!! else removed.right!!
        successor.parent = removed.parent

        if (removed.parent == null) {
            root = successor
        } else if (removed === removed.parent!!.left) {
            removed.parent!!.left = successor
        } else {
            removed.parent!!.right = successor
        }

        if (removed.color == RBT_BLACK) {
            rebuildAfterRemove(successor)
        }
        return removed
    }

    fun inorder(): List<Int> = buildList { inorderHelper(root, this) }

    // ─── 내부 구현 ─────────────────────────────────────────────────────────────

    private fun insertHelper(newNode: RBTNode) {
        if (root == null) { root = newNode; return }
        var curr = root!!
        while (true) {
            when {
                curr.data < newNode.data -> {
                    if (curr.right === RBT_NIL) { curr.right = newNode; newNode.parent = curr; return }
                    curr = curr.right!!
                }
                curr.data > newNode.data -> {
                    if (curr.left === RBT_NIL) { curr.left = newNode; newNode.parent = curr; return }
                    curr = curr.left!!
                }
                else -> return // 중복, 삽입하지 않음
            }
        }
    }

    private fun searchHelper(node: RBTNode?, target: Int): RBTNode? {
        if (node == null || node === RBT_NIL) return null
        return when {
            node.data == target -> node
            node.data > target -> searchHelper(node.left, target)
            else -> searchHelper(node.right, target)
        }
    }

    private fun rotateLeft(parent: RBTNode) {
        val rightChild = parent.right!!
        parent.right = rightChild.left
        if (rightChild.left !== RBT_NIL) rightChild.left!!.parent = parent
        rightChild.parent = parent.parent
        when {
            parent.parent == null -> root = rightChild
            parent === parent.parent!!.left -> parent.parent!!.left = rightChild
            else -> parent.parent!!.right = rightChild
        }
        rightChild.left = parent
        parent.parent = rightChild
    }

    private fun rotateRight(parent: RBTNode) {
        val leftChild = parent.left!!
        parent.left = leftChild.right
        if (leftChild.right !== RBT_NIL) leftChild.right!!.parent = parent
        leftChild.parent = parent.parent
        when {
            parent.parent == null -> root = leftChild
            parent === parent.parent!!.left -> parent.parent!!.left = leftChild
            else -> parent.parent!!.right = leftChild
        }
        leftChild.right = parent
        parent.parent = leftChild
    }

    private fun rebuildAfterInsert(startX: RBTNode) {
        var x = startX
        while (x !== root && x.parent!!.color == RBT_RED) {
            val parent = x.parent!!
            val grandparent = parent.parent!!
            if (parent === grandparent.left) {
                val uncle = grandparent.right!!
                if (uncle.color == RBT_RED) {
                    parent.color = RBT_BLACK
                    uncle.color = RBT_BLACK
                    grandparent.color = RBT_RED
                    x = grandparent
                } else {
                    if (x === parent.right) {
                        x = parent
                        rotateLeft(x)
                    }
                    x.parent!!.color = RBT_BLACK
                    x.parent!!.parent!!.color = RBT_RED
                    rotateRight(x.parent!!.parent!!)
                }
            } else {
                val uncle = grandparent.left!!
                if (uncle.color == RBT_RED) {
                    parent.color = RBT_BLACK
                    uncle.color = RBT_BLACK
                    grandparent.color = RBT_RED
                    x = grandparent
                } else {
                    if (x === parent.left) {
                        x = parent
                        rotateRight(x)
                    }
                    x.parent!!.color = RBT_BLACK
                    x.parent!!.parent!!.color = RBT_RED
                    rotateLeft(x.parent!!.parent!!)
                }
            }
        }
        root!!.color = RBT_BLACK
    }

    private fun rebuildAfterRemove(startSuccessor: RBTNode) {
        var successor = startSuccessor
        while (successor.parent != null && successor.color == RBT_BLACK) {
            val parent = successor.parent!!
            if (successor === parent.left) {
                var sibling = parent.right!!
                if (sibling.color == RBT_RED) {
                    sibling.color = RBT_BLACK
                    parent.color = RBT_RED
                    rotateLeft(parent)
                } else if (sibling.left!!.color == RBT_BLACK && sibling.right!!.color == RBT_BLACK) {
                    sibling.color = RBT_RED
                    successor = parent
                } else {
                    if (sibling.left!!.color == RBT_RED) {
                        sibling.left!!.color = RBT_BLACK
                        sibling.color = RBT_RED
                        rotateRight(sibling)
                        sibling = successor.parent!!.right!!
                    }
                    sibling.color = successor.parent!!.color
                    successor.parent!!.color = RBT_BLACK
                    sibling.right!!.color = RBT_BLACK
                    rotateLeft(successor.parent!!)
                    successor = root!!
                }
            } else {
                var sibling = parent.left!!
                if (sibling.color == RBT_RED) {
                    sibling.color = RBT_BLACK
                    parent.color = RBT_RED
                    rotateRight(parent)
                } else if (sibling.right!!.color == RBT_BLACK && sibling.left!!.color == RBT_BLACK) {
                    sibling.color = RBT_RED
                    successor = parent
                } else {
                    if (sibling.right!!.color == RBT_RED) {
                        sibling.right!!.color = RBT_BLACK
                        sibling.color = RBT_RED
                        rotateLeft(sibling)
                        sibling = successor.parent!!.left!!
                    }
                    sibling.color = successor.parent!!.color
                    successor.parent!!.color = RBT_BLACK
                    sibling.left!!.color = RBT_BLACK
                    rotateRight(successor.parent!!)
                    successor = root!!
                }
            }
        }
        successor.color = RBT_BLACK
    }

    private fun inorderHelper(node: RBTNode?, out: MutableList<Int>) {
        if (node == null || node === RBT_NIL) return
        inorderHelper(node.left, out)
        out.add(node.data)
        inorderHelper(node.right, out)
    }
}

fun main() {
    val rbt = RedBlackTree()
    listOf(10, 20, 30, 15, 25, 5, 1, 40, 35, 50).forEach { rbt.insert(RBTNode(it)) }
    println("Inorder: ${rbt.inorder()}")
    println("Root color: ${if (rbt.root!!.color == RBT_BLACK) "BLACK" else "RED"}")
}
