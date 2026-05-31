package ch04

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotSame
import kotlin.test.assertSame
import kotlin.test.assertTrue

class BinaryTreeTest {
    //        A
    //      /   \
    //     B     E
    //    / \   / \
    //   C   D F   G
    private fun buildTree(): BinaryNode {
        val b = BinaryNode('B').also {
            it.left = BinaryNode('C')
            it.right = BinaryNode('D')
        }
        val e = BinaryNode('E').also {
            it.left = BinaryNode('F')
            it.right = BinaryNode('G')
        }
        return BinaryNode('A').also {
            it.left = b
            it.right = e
        }
    }

    @Test
    fun preorderTraversal() {
        assertEquals(listOf('A', 'B', 'C', 'D', 'E', 'F', 'G'), preorder(buildTree()))
    }

    @Test
    fun inorderTraversal() {
        assertEquals(listOf('C', 'B', 'D', 'A', 'F', 'E', 'G'), inorder(buildTree()))
    }

    @Test
    fun postorderTraversal() {
        assertEquals(listOf('C', 'D', 'B', 'F', 'G', 'E', 'A'), postorder(buildTree()))
    }
}

class LCRSTreeTest {
    @Test
    fun addChildAppendsAsSibling() {
        val root = LCRSNode('A')
        val b = LCRSNode('B'); val g = LCRSNode('G'); val i = LCRSNode('I')
        addChildNode(root, b); addChildNode(root, g); addChildNode(root, i)

        val children = mutableListOf<Char>()
        var c = root.leftChild
        while (c != null) { children.add(c.data); c = c.rightSibling }
        assertEquals(listOf('B', 'G', 'I'), children)
    }

    @Test
    fun nestedChildren() {
        val root = LCRSNode('A')
        val b = LCRSNode('B'); val c = LCRSNode('C'); val d = LCRSNode('D')
        addChildNode(root, b); addChildNode(b, c); addChildNode(b, d)

        val rootChildren = mutableListOf<Char>()
        var node = root.leftChild
        while (node != null) { rootChildren.add(node.data); node = node.rightSibling }
        assertEquals(listOf('B'), rootChildren)

        val bChildren = mutableListOf<Char>()
        node = b.leftChild
        while (node != null) { bChildren.add(node.data); node = node.rightSibling }
        assertEquals(listOf('C', 'D'), bChildren)
    }
}

class ExpressionTreeTest {
    @Test
    fun evaluateSample() {
        // "71*52-/" = (7*1) / (5-2) = 7/3
        val postfix = ArrayDeque("71*52-/".toList())
        val root = buildExpressionTree(postfix)
        assertTrue((evaluateET(root) - 7.0 / 3.0) < 1e-9)
    }

    @Test
    fun evaluateSimpleOps() {
        assertEquals(7.0, evaluateET(buildExpressionTree(ArrayDeque("34+".toList()))))
        assertEquals(6.0, evaluateET(buildExpressionTree(ArrayDeque("82-".toList()))))
        assertEquals(18.0, evaluateET(buildExpressionTree(ArrayDeque("63*".toList()))))
    }

    @Test
    fun buildPreorderStructure() {
        val postfix = ArrayDeque("71*52-/".toList())
        val root = buildExpressionTree(postfix)

        fun pre(node: ETNode?, out: MutableList<Char>) {
            if (node != null) { out.add(node.data); pre(node.left, out); pre(node.right, out) }
        }
        val out = mutableListOf<Char>()
        pre(root, out)
        assertEquals(listOf('/', '*', '7', '1', '-', '5', '2'), out)
    }
}

class DisjointSetTest {
    @Test
    fun separateSetsAreDisjoint() {
        val s1 = DisjointSet(1); val s2 = DisjointSet(2)
        assertNotSame(s1.findRoot(), s2.findRoot())
    }

    @Test
    fun unionMergesSets() {
        val s1 = DisjointSet(1); val s2 = DisjointSet(2); val s3 = DisjointSet(3)
        dsUnion(s1, s3)
        assertSame(s1.findRoot(), s3.findRoot())
        assertNotSame(s1.findRoot(), s2.findRoot())
    }

    @Test
    fun unionIsTransitive() {
        val s1 = DisjointSet(1); val s3 = DisjointSet(3); val s4 = DisjointSet(4)
        dsUnion(s1, s3)
        dsUnion(s3, s4)
        assertSame(s3.findRoot(), s4.findRoot())
        assertSame(s1.findRoot(), s4.findRoot())
    }
}
