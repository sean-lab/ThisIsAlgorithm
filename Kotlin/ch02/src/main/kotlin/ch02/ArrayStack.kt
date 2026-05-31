package ch02

/**
 * 배열 기반 스택 (ArrayStack)
 * Rust/ch02/src/array_stack.rs → Kotlin 이디엄 포팅
 */
class ArrayStack(val capacity: Int) {

    private val nodes = IntArray(capacity)
    private var top = -1

    fun push(data: Int) {
        if (top >= capacity - 1) {
            println("Stack Overflow!")
            return
        }
        nodes[++top] = data
    }

    fun pop(): Int {
        if (top < 0) {
            println("Stack Underflow!")
            return 0
        }
        return nodes[top--]
    }

    fun top(): Int {
        if (top < 0) {
            println("Stack is empty!")
            return 0
        }
        return nodes[top]
    }

    fun getSize(): Int = top + 1

    fun isEmpty(): Boolean = top == -1
}

fun main() {
    val stack = ArrayStack(10)
    for (i in 1..6) stack.push(i * 10)

    println("Capacity: ${stack.capacity}")
    println("Top: ${stack.top()}")

    while (!stack.isEmpty()) println("Popped: ${stack.pop()}")
}
