// 단순 해시 테이블 (Simple Hash Table)
// Clang/08/SimpleHashTable 포팅. 직접 주소화이므로 Vec 사용.
type KeyType = i32;
type ValueType = i32;

#[derive(Clone, Copy, Default)]
struct Node {
    key: KeyType,
    value: ValueType,
}

struct HashTable {
    table_size: i32,
    table: Vec<Node>,
}

impl HashTable {
    fn create(table_size: i32) -> HashTable {
        HashTable {
            table_size,
            table: vec![Node::default(); table_size as usize],
        }
    }

    fn hash(key: KeyType, table_size: i32) -> i32 {
        key % table_size
    }

    fn set(&mut self, key: KeyType, value: ValueType) {
        let address = HashTable::hash(key, self.table_size);
        self.table[address as usize].key = key;
        self.table[address as usize].value = value;
    }

    fn get(&self, key: KeyType) -> ValueType {
        let address = HashTable::hash(key, self.table_size);
        self.table[address as usize].value
    }
}

fn main() {
    let mut ht = HashTable::create(193);

    ht.set(418, 32114);
    ht.set(9, 514);
    ht.set(27, 8917);
    ht.set(1031, 286);

    println!("Key:{}, Value:{}", 418, ht.get(418));
    println!("Key:{}, Value:{}", 9, ht.get(9));
    println!("Key:{}, Value:{}", 27, ht.get(27));
    println!("Key:{}, Value:{}", 1031, ht.get(1031));
}
