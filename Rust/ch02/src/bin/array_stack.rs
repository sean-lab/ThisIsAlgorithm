// 배열 기반 스택 (ArrayStack)
// Clang/02/ArrayStack 포팅. 로직은 ch02::array_stack 모듈에 있다.
use ch02::array_stack::ArrayStack;

fn main() {
    let mut stack = ArrayStack::create(10);

    stack.push(3);
    stack.push(37);
    stack.push(11);
    stack.push(12);

    println!(
        "Capacity: {}, Size: {}, Top: {}\n",
        stack.capacity,
        stack.get_size(),
        stack.top()
    );

    for _ in 0..4 {
        if stack.is_empty() {
            break;
        }

        print!("Popped: {}, ", stack.pop());

        if !stack.is_empty() {
            println!("Current Top: {}", stack.top());
        } else {
            println!("Stack Is Empty.");
        }
    }
}
