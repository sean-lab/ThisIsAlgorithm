// Huffman coding. Port of Clang/14/Huffman/{Huffman.c, PriorityQueue.c} +
// Test_Huffman.c. Uses raw pointers for the prefix tree and the type-erased
// priority queue (PQNode.Data is a void* to a HuffmanNode), faithfully
// reproducing the C tie-breaking so the encoded bit stream is identical.

use std::os::raw::c_void;
use std::ptr;

const MAX_CHAR: usize = 256;
const MAX_BIT: usize = 8;

#[derive(Clone, Copy)]
struct SymbolInfo {
    symbol: u8,
    frequency: i32,
}

struct HuffmanNode {
    data: SymbolInfo,
    left: *mut HuffmanNode,
    right: *mut HuffmanNode,
}

struct BitBuffer {
    buffer: Vec<u8>,
    size: u32,
}

#[derive(Clone, Copy)]
struct HuffmanCode {
    code: [u8; MAX_BIT],
    size: i32,
}

#[derive(Clone, Copy)]
struct PQNode {
    priority: i32,
    data: *mut c_void,
}

struct PriorityQueue {
    nodes: Vec<PQNode>,
    used_size: i32,
}

fn pq_get_parent(index: i32) -> i32 {
    (index - 1) / 2
}

fn pq_get_left_child(index: i32) -> i32 {
    (2 * index) + 1
}

impl PriorityQueue {
    fn create() -> PriorityQueue {
        PriorityQueue {
            nodes: Vec::new(),
            used_size: 0,
        }
    }

    fn enqueue(&mut self, new_node: PQNode) {
        let mut current_position = self.used_size;
        let mut parent_position = pq_get_parent(current_position);

        if (self.nodes.len() as i32) <= current_position {
            self.nodes.push(PQNode {
                priority: 0,
                data: ptr::null_mut(),
            });
        }

        self.nodes[current_position as usize] = new_node;

        while current_position > 0
            && self.nodes[current_position as usize].priority
                < self.nodes[parent_position as usize].priority
        {
            self.nodes
                .swap(current_position as usize, parent_position as usize);
            current_position = parent_position;
            parent_position = pq_get_parent(current_position);
        }

        self.used_size += 1;
    }

    fn dequeue(&mut self) -> PQNode {
        let root = self.nodes[0];

        self.used_size -= 1;
        self.nodes.swap(0, self.used_size as usize);

        let mut parent_position = 0;
        let mut left_position = pq_get_left_child(0);
        let mut right_position = left_position + 1;

        loop {
            let selected_child;

            if left_position >= self.used_size {
                break;
            }

            if right_position >= self.used_size {
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
                self.nodes
                    .swap(parent_position as usize, selected_child as usize);
                parent_position = selected_child;
            } else {
                break;
            }

            left_position = pq_get_left_child(parent_position);
            right_position = left_position + 1;
        }

        root
    }
}

fn huffman_create_node(new_data: SymbolInfo) -> *mut HuffmanNode {
    Box::into_raw(Box::new(HuffmanNode {
        data: new_data,
        left: ptr::null_mut(),
        right: ptr::null_mut(),
    }))
}

unsafe fn huffman_destroy_tree(node: *mut HuffmanNode) {
    if node.is_null() {
        return;
    }
    huffman_destroy_tree((*node).left);
    huffman_destroy_tree((*node).right);
    drop(Box::from_raw(node));
}

fn huffman_add_bit(buffer: &mut BitBuffer, bit: u8) {
    let mut mask: u8 = 0x80;

    if buffer.size % 8 == 0 {
        buffer.buffer.push(0x00);
    }

    mask >>= buffer.size % 8;

    let idx = (buffer.size / 8) as usize;
    if bit == 1 {
        buffer.buffer[idx] |= mask;
    } else {
        buffer.buffer[idx] &= !mask;
    }

    buffer.size += 1;
}

unsafe fn huffman_build_code_table(
    tree: *mut HuffmanNode,
    code_table: &mut [HuffmanCode],
    code: &mut [u8],
    size: usize,
) {
    if tree.is_null() {
        return;
    }

    if !(*tree).left.is_null() {
        code[size] = 0;
        huffman_build_code_table((*tree).left, code_table, code, size + 1);
    }

    if !(*tree).right.is_null() {
        code[size] = 1;
        huffman_build_code_table((*tree).right, code_table, code, size + 1);
    }

    if (*tree).left.is_null() && (*tree).right.is_null() {
        let symbol = (*tree).data.symbol as usize;
        for i in 0..size {
            code_table[symbol].code[i] = code[i];
        }
        code_table[symbol].size = size as i32;
    }
}

unsafe fn huffman_build_prefix_tree(tree: &mut *mut HuffmanNode, symbol_info_table: &[SymbolInfo]) {
    let mut pq = PriorityQueue::create();

    for i in 0..MAX_CHAR {
        if symbol_info_table[i].frequency > 0 {
            let bit_node = huffman_create_node(symbol_info_table[i]);
            let new_node = PQNode {
                priority: symbol_info_table[i].frequency,
                data: bit_node as *mut c_void,
            };
            pq.enqueue(new_node);
        }
    }

    while pq.used_size > 1 {
        let new_data = SymbolInfo {
            symbol: 0,
            frequency: 0,
        };
        let bit_node = huffman_create_node(new_data);

        let q_left = pq.dequeue();
        let q_right = pq.dequeue();

        let left = q_left.data as *mut HuffmanNode;
        let right = q_right.data as *mut HuffmanNode;

        (*bit_node).data.symbol = 0;
        (*bit_node).data.frequency = (*left).data.frequency + (*right).data.frequency;

        (*bit_node).left = left;
        (*bit_node).right = right;

        let new_node = PQNode {
            priority: (*bit_node).data.frequency,
            data: bit_node as *mut c_void,
        };
        pq.enqueue(new_node);
    }

    let result = pq.dequeue();
    *tree = result.data as *mut HuffmanNode;
}

unsafe fn huffman_encode(
    tree: &mut *mut HuffmanNode,
    source: &[u8],
    encoded: &mut BitBuffer,
    code_table: &mut [HuffmanCode],
) {
    let mut symbol_info_table = vec![
        SymbolInfo {
            symbol: 0,
            frequency: 0
        };
        MAX_CHAR
    ];
    let mut temporary = [0u8; MAX_BIT];

    for i in 0..MAX_CHAR {
        symbol_info_table[i].symbol = i as u8;
        symbol_info_table[i].frequency = 0;
    }

    let mut i = 0;
    while source[i] != b'\0' {
        symbol_info_table[source[i] as usize].frequency += 1;
        i += 1;
    }

    huffman_build_prefix_tree(tree, &symbol_info_table);

    huffman_build_code_table(*tree, code_table, &mut temporary, 0);

    let mut i = 0;
    while source[i] != b'\0' {
        let bit_count = code_table[source[i] as usize].size;

        for j in 0..bit_count as usize {
            huffman_add_bit(encoded, code_table[source[i] as usize].code[j]);
        }

        i += 1;
    }
}

unsafe fn huffman_decode(tree: *mut HuffmanNode, encoded: &BitBuffer, decoded: &mut Vec<u8>) {
    let mut index = 0;
    let mut current = tree;

    for i in 0..=encoded.size {
        let mut mask: u8 = 0x80;

        if (*current).left.is_null() && (*current).right.is_null() {
            decoded[index] = (*current).data.symbol;
            index += 1;
            current = tree;
        }

        mask >>= i % 8;

        // The original loop runs `i <= Size`; on the final iteration this can
        // read one byte past the buffer when Size is a multiple of 8. That
        // read is harmless (the last symbol has already been emitted), so we
        // treat an out-of-range byte as 0 instead of reading out of bounds.
        let byte_index = (i / 8) as usize;
        let byte = encoded.buffer.get(byte_index).copied().unwrap_or(0);

        if (byte & mask) != mask {
            current = (*current).left;
        } else {
            current = (*current).right;
        }
    }

    decoded[index] = b'\0';
}

fn huffman_binary_string(buffer: &BitBuffer) -> String {
    let mut s = String::new();
    for i in 0..buffer.size {
        let mut mask: u8 = 0x80;
        mask >>= i % 8;

        let bit = (buffer.buffer[(i / 8) as usize] & mask) == mask;
        s.push_str(&(bit as i32).to_string());
    }
    s
}

/// 허프만 인코딩/디코딩 결과. `source` 는 NUL(`\0`) 종단 바이트열이어야 한다.
pub struct HuffmanResult {
    pub original_size: usize,
    pub encoded_size: u32,
    pub original: String,
    pub binary: String,
    pub decoded: String,
}

pub fn run(source: &[u8]) -> HuffmanResult {
    let mut tree: *mut HuffmanNode = ptr::null_mut();
    let mut encoded = BitBuffer {
        buffer: Vec::new(),
        size: 0,
    };
    let mut code_table = vec![
        HuffmanCode {
            code: [0; MAX_BIT],
            size: 0
        };
        MAX_CHAR
    ];

    unsafe {
        huffman_encode(&mut tree, source, &mut encoded, &mut code_table);

        let strlen = source.len() - 1; // exclude the NUL terminator

        let mut decoded = vec![0u8; strlen + 1];
        huffman_decode(tree, &encoded, &mut decoded);

        let decoded_str = {
            let end = decoded.iter().position(|&b| b == 0).unwrap_or(decoded.len());
            String::from_utf8_lossy(&decoded[..end]).into_owned()
        };

        let binary = huffman_binary_string(&encoded);
        let result = HuffmanResult {
            original_size: (strlen + 1) * 8,
            encoded_size: encoded.size,
            original: String::from_utf8_lossy(&source[..strlen]).into_owned(),
            binary,
            decoded: decoded_str,
        };

        huffman_destroy_tree(tree);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_recovers_original() {
        let r = run(b"This Is Algorithms.\0");
        assert_eq!(r.decoded, "This Is Algorithms.");
        assert_eq!(r.original, "This Is Algorithms.");
    }

    #[test]
    fn encoded_is_smaller_than_original() {
        let r = run(b"This Is Algorithms.\0");
        assert_eq!(r.original_size, 20 * 8);
        assert!(r.encoded_size < r.original_size as u32);
    }

    #[test]
    fn binary_length_equals_encoded_size() {
        let r = run(b"This Is Algorithms.\0");
        assert_eq!(r.binary.len() as u32, r.encoded_size);
    }
}
