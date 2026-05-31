package ch04

/**
 * 수식 트리 (Expression Tree)
 * Rust/ch04/src/expression_tree.rs → Kotlin 이디엄 포팅
 * 후위 표기식으로부터 트리를 구성하고 평가한다.
 */
class ETNode(val data: Char) {
    var left: ETNode? = null
    var right: ETNode? = null
}

/**
 * 후위 표기식 배열의 뒤에서부터 토큰을 꺼내며 수식 트리를 구성한다.
 * postfix 배열은 소비된다 (removeLast 사용).
 */
fun buildExpressionTree(postfix: ArrayDeque<Char>): ETNode? {
    if (postfix.isEmpty()) return null
    val token = postfix.removeLast()
    return when (token) {
        '+', '-', '*', '/' -> {
            val node = ETNode(token)
            node.right = buildExpressionTree(postfix)   // 오른쪽 먼저
            node.left = buildExpressionTree(postfix)    // 왼쪽 나중
            node
        }
        else -> ETNode(token)
    }
}

fun evaluateET(tree: ETNode?): Double {
    tree ?: return 0.0
    return when (tree.data) {
        '+', '-', '*', '/' -> {
            val left = evaluateET(tree.left)
            val right = evaluateET(tree.right)
            when (tree.data) {
                '+' -> left + right
                '-' -> left - right
                '*' -> left * right
                '/' -> left / right
                else -> 0.0
            }
        }
        else -> tree.data.toString().toDoubleOrNull() ?: 0.0
    }
}

fun main() {
    // "71*52-/" = (7*1) / (5-2)
    val postfix = ArrayDeque("71*52-/".toList())
    val root = buildExpressionTree(postfix)
    println("Result: ${evaluateET(root)}")
}
