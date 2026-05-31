// 배열 기반 스택 (ArrayStack)
// Clang/02/ArrayStack 포팅.

type ElementType = i32;

struct ArrayStack {
    capacity: i32,
    top: i32,
    nodes: Vec<ElementType>,
}

impl ArrayStack {
    fn create(capacity: i32) -> ArrayStack {
        ArrayStack {
            capacity,
            top: -1,
            nodes: vec![0; capacity as usize],
        }
    }

    fn push(&mut self, data: ElementType) {
        if self.top >= self.capacity - 1 {
            // 스택 오버플로우 방지
            println!("Stack Overflow!");
            return;
        }
        self.top += 1;
        self.nodes[self.top as usize] = data;
    }

    fn pop(&mut self) -> ElementType {
        if self.top < 0 {
            // 스택 언더플로우 방지
            println!("Stack Underflow!");
            return 0;
        }
        let position = self.top;
        self.top -= 1;
        self.nodes[position as usize]
    }

    fn top(&self) -> ElementType {
        if self.top < 0 {
            println!("Stack is empty!");
            return 0;
        }
        self.nodes[self.top as usize]
    }

    fn get_size(&self) -> i32 {
        self.top + 1
    }

    fn is_empty(&self) -> bool {
        self.top == -1
    }
}

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
