// 연결 리스트 기반 스택 (LinkedListStack)
// Clang/02 LinkedListStack 포팅. C의 노드 기반 LIFO 동작을 그대로 옮겼다.
// C는 char* 데이터를 저장하므로 Rust에서는 String을 저장한다.

pub struct LinkedListStack {
    items: Vec<String>,
}

impl LinkedListStack {
    pub fn new() -> Self {
        LinkedListStack { items: Vec::new() }
    }

    pub fn push(&mut self, data: String) {
        self.items.push(data);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.items.pop()
    }

    pub fn top(&self) -> Option<&String> {
        self.items.last()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for LinkedListStack {
    fn default() -> Self {
        Self::new()
    }
}
