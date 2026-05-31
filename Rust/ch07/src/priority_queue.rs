// 우선순위 큐 (Priority Queue)
// Clang/07/PriorityQueue 포팅. 배열 기반 최소 힙이므로 Vec 사용.
pub type PriorityType = i32;

#[derive(Clone, Default)]
pub struct PQNode {
    pub priority: PriorityType,
    pub data: String,
}

pub struct PriorityQueue {
    pub nodes: Vec<PQNode>,
    pub capacity: usize,
    pub used_size: usize,
}

impl PriorityQueue {
    pub fn create(initial_size: usize) -> PriorityQueue {
        PriorityQueue {
            nodes: vec![PQNode::default(); initial_size],
            capacity: initial_size,
            used_size: 0,
        }
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

    pub fn enqueue(&mut self, new_node: PQNode) {
        let mut current_position = self.used_size as i32;
        let mut parent_position = PriorityQueue::get_parent(current_position);

        if self.used_size == self.capacity {
            if self.capacity == 0 {
                self.capacity = 1;
            }
            self.capacity *= 2;
            self.nodes.resize(self.capacity, PQNode::default());
        }

        self.nodes[current_position as usize] = new_node;

        while current_position > 0
            && self.nodes[current_position as usize].priority
                < self.nodes[parent_position as usize].priority
        {
            self.swap_nodes(current_position, parent_position);

            current_position = parent_position;
            parent_position = PriorityQueue::get_parent(current_position);
        }

        self.used_size += 1;
    }

    pub fn dequeue(&mut self, root: &mut PQNode) {
        let mut parent_position: i32 = 0;
        let mut left_position: i32;
        let mut right_position: i32;

        *root = std::mem::take(&mut self.nodes[0]);

        self.used_size -= 1;
        self.swap_nodes(0, self.used_size as i32);

        left_position = PriorityQueue::get_left_child(0);
        right_position = left_position + 1;

        loop {
            let selected_child;

            if left_position >= self.used_size as i32 {
                break;
            }

            if right_position >= self.used_size as i32 {
                selected_child = left_position;
            } else if self.nodes[left_position as usize].priority
                > self.nodes[right_position as usize].priority
            {
                selected_child = right_position;
            } else {
                selected_child = left_position;
            }

            if self.nodes[selected_child as usize].priority
                < self.nodes[parent_position as usize].priority
            {
                self.swap_nodes(parent_position, selected_child);
                parent_position = selected_child;
            } else {
                break;
            }

            left_position = PriorityQueue::get_left_child(parent_position);
            right_position = left_position + 1;
        }

        if self.used_size < (self.capacity / 2) {
            self.capacity /= 2;
            self.nodes.resize(self.capacity, PQNode::default());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.used_size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dequeue_returns_priority_ascending() {
        let mut pq = PriorityQueue::create(3);
        let inputs = [
            (34, "코딩"),
            (12, "고객미팅"),
            (87, "커피타기"),
            (45, "문서작성"),
            (35, "디버깅"),
            (66, "이닦기"),
        ];
        for (p, d) in inputs {
            pq.enqueue(PQNode {
                priority: p,
                data: d.to_string(),
            });
        }
        assert_eq!(pq.used_size, 6);

        let mut out = Vec::new();
        let mut popped = PQNode::default();
        while !pq.is_empty() {
            pq.dequeue(&mut popped);
            out.push((popped.priority, popped.data.clone()));
        }
        assert_eq!(
            out,
            vec![
                (12, "고객미팅".to_string()),
                (34, "코딩".to_string()),
                (35, "디버깅".to_string()),
                (45, "문서작성".to_string()),
                (66, "이닦기".to_string()),
                (87, "커피타기".to_string()),
            ]
        );
        assert!(pq.is_empty());
    }
}
