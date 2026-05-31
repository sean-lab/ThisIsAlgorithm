// 배열 기반 스택 (ArrayStack)
// Clang/02/ArrayStack 포팅.

pub type ElementType = i32;

pub struct ArrayStack {
    pub capacity: i32,
    pub top: i32,
    pub nodes: Vec<ElementType>,
}

impl ArrayStack {
    pub fn create(capacity: i32) -> ArrayStack {
        ArrayStack {
            capacity,
            top: -1,
            nodes: vec![0; capacity as usize],
        }
    }

    pub fn push(&mut self, data: ElementType) {
        if self.top >= self.capacity - 1 {
            // 스택 오버플로우 방지
            println!("Stack Overflow!");
            return;
        }
        self.top += 1;
        self.nodes[self.top as usize] = data;
    }

    pub fn pop(&mut self) -> ElementType {
        if self.top < 0 {
            // 스택 언더플로우 방지
            println!("Stack Underflow!");
            return 0;
        }
        let position = self.top;
        self.top -= 1;
        self.nodes[position as usize]
    }

    pub fn top(&self) -> ElementType {
        if self.top < 0 {
            println!("Stack is empty!");
            return 0;
        }
        self.nodes[self.top as usize]
    }

    pub fn get_size(&self) -> i32 {
        self.top + 1
    }

    pub fn is_empty(&self) -> bool {
        self.top == -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_is_lifo() {
        let mut s = ArrayStack::create(10);
        s.push(3);
        s.push(37);
        s.push(11);
        assert_eq!(s.get_size(), 3);
        assert_eq!(s.top(), 11);
        assert_eq!(s.pop(), 11);
        assert_eq!(s.pop(), 37);
        assert_eq!(s.pop(), 3);
        assert!(s.is_empty());
    }

    #[test]
    fn overflow_does_not_push() {
        let mut s = ArrayStack::create(2);
        s.push(1);
        s.push(2);
        s.push(3); // 용량 초과 -> 무시
        assert_eq!(s.get_size(), 2);
        assert_eq!(s.top(), 2);
    }

    #[test]
    fn underflow_returns_zero() {
        let mut s = ArrayStack::create(4);
        assert!(s.is_empty());
        assert_eq!(s.pop(), 0);
        assert_eq!(s.top(), 0);
    }

    #[test]
    fn empty_after_creation() {
        let s = ArrayStack::create(5);
        assert!(s.is_empty());
        assert_eq!(s.get_size(), 0);
        assert_eq!(s.capacity, 5);
    }
}
