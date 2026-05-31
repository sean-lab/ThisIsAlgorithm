// 힙 (Heap) - 최소 힙
// Clang/07/Heap 포팅. 배열 기반이므로 Vec 사용.
use std::mem;

type ElementType = i32;

#[derive(Clone, Copy, Default)]
struct HeapNode {
    data: ElementType,
}

struct Heap {
    nodes: Vec<HeapNode>,
    capacity: usize,
    used_size: usize,
}

impl Heap {
    fn create(initial_size: usize) -> Heap {
        let h = Heap {
            nodes: vec![HeapNode::default(); initial_size],
            capacity: initial_size,
            used_size: 0,
        };
        // 원본 C: printf("size : %d\n", sizeof(HeapNode));
        println!("size : {}", mem::size_of::<HeapNode>());
        h
    }

    fn get_parent(index: i32) -> i32 {
        (index - 1) / 2
    }

    fn get_left_child(index: i32) -> i32 {
        (2 * index) + 1
    }

    fn swap_nodes(&mut self, index1: i32, index2: i32) {
        self.nodes.swap(index1 as usize, index2 as usize);
    }

    fn insert(&mut self, new_data: ElementType) {
        let mut current_position = self.used_size as i32;
        let mut parent_position = Heap::get_parent(current_position);

        if self.used_size == self.capacity {
            self.capacity *= 2;
            self.nodes.resize(self.capacity, HeapNode::default());
        }

        self.nodes[current_position as usize].data = new_data;

        while current_position > 0
            && self.nodes[current_position as usize].data
                < self.nodes[parent_position as usize].data
        {
            self.swap_nodes(current_position, parent_position);

            current_position = parent_position;
            parent_position = Heap::get_parent(current_position);
        }

        self.used_size += 1;
    }

    fn delete_min(&mut self, root: &mut HeapNode) {
        let mut parent_position: i32 = 0;
        let mut left_position: i32;
        let mut right_position: i32;

        *root = self.nodes[0];
        self.nodes[0] = HeapNode::default();

        self.used_size -= 1;
        self.swap_nodes(0, self.used_size as i32);

        left_position = Heap::get_left_child(0);
        right_position = left_position + 1;

        loop {
            let selected_child;

            if left_position >= self.used_size as i32 {
                break;
            }

            if right_position >= self.used_size as i32 {
                selected_child = left_position;
            } else if self.nodes[left_position as usize].data
                > self.nodes[right_position as usize].data
            {
                selected_child = right_position;
            } else {
                selected_child = left_position;
            }

            if self.nodes[selected_child as usize].data
                < self.nodes[parent_position as usize].data
            {
                self.swap_nodes(parent_position, selected_child);
                parent_position = selected_child;
            } else {
                break;
            }

            left_position = Heap::get_left_child(parent_position);
            right_position = left_position + 1;
        }

        if self.used_size < (self.capacity / 2) {
            self.capacity /= 2;
            self.nodes.resize(self.capacity, HeapNode::default());
        }
    }

    fn print_nodes(&self) {
        for i in 0..self.used_size {
            print!("{} ", self.nodes[i].data);
        }
        println!();
    }
}

fn main() {
    let mut h = Heap::create(3);
    let mut min_node = HeapNode::default();

    h.insert(12);
    h.insert(87);
    h.insert(111);
    h.insert(34);
    h.insert(16);
    h.insert(75);
    h.print_nodes();

    for _ in 0..6 {
        h.delete_min(&mut min_node);
        h.print_nodes();
    }
}
