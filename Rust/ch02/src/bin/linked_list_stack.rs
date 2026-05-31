// 연결 리스트 스택 테스트
// Clang/02/LinkedListStack/Test_LinkedListStack.c 포팅.
use ch02::linked_list_stack::LinkedListStack;

fn main() {
    let mut stack = LinkedListStack::new();

    stack.push("abc".to_string());
    stack.push("def".to_string());
    stack.push("efg".to_string());
    stack.push("hij".to_string());

    let count = stack.size();
    println!("Size: {}, Top: {}\n", count, stack.top().unwrap());

    for _ in 0..count {
        if stack.is_empty() {
            break;
        }

        let popped = stack.pop().unwrap();
        print!("Popped: {}, ", popped);

        if !stack.is_empty() {
            println!("Current Top: {}", stack.top().unwrap());
        } else {
            println!("Stack Is Empty.");
        }
    }
}
