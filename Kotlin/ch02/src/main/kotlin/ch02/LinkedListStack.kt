package ch02

/**
 * 연결 리스트 기반 스택 (LinkedListStack)
 * Rust/ch02/src/linked_list_stack.rs → Kotlin 이디엄 포팅
 * 책의 원본 C 코드는 char* 데이터를 저장하므로 String을 저장한다.
 */
class LinkedListStack {

    private val items = ArrayDeque<String>()

    fun push(data: String) = items.addLast(data)

    fun pop(): String? = items.removeLastOrNull()

    fun top(): String? = items.lastOrNull()

    fun size(): Int = items.size

    fun isEmpty(): Boolean = items.isEmpty()
}

fun main() {
    val stack = LinkedListStack()
    stack.push("abc")
    stack.push("def")
    stack.push("efg")

    while (!stack.isEmpty()) println("Popped: ${stack.pop()}")
}
