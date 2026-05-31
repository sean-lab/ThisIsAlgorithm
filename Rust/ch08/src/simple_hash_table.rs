// 단순 해시 테이블 (Simple Hash Table)
// Clang/08/SimpleHashTable 포팅. 직접 주소화이므로 Vec 사용.
pub type KeyType = i32;
pub type ValueType = i32;

#[derive(Clone, Copy, Default)]
pub struct Node {
    pub key: KeyType,
    pub value: ValueType,
}

pub struct HashTable {
    pub table_size: i32,
    pub table: Vec<Node>,
}

impl HashTable {
    pub fn create(table_size: i32) -> HashTable {
        HashTable {
            table_size,
            table: vec![Node::default(); table_size as usize],
        }
    }

    pub fn hash(key: KeyType, table_size: i32) -> i32 {
        key % table_size
    }

    pub fn set(&mut self, key: KeyType, value: ValueType) {
        let address = HashTable::hash(key, self.table_size);
        self.table[address as usize].key = key;
        self.table[address as usize].value = value;
    }

    pub fn get(&self, key: KeyType) -> ValueType {
        let address = HashTable::hash(key, self.table_size);
        self.table[address as usize].value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_then_get_returns_value() {
        let mut ht = HashTable::create(193);
        ht.set(418, 32114);
        ht.set(9, 514);
        ht.set(27, 8917);
        ht.set(1031, 286);
        assert_eq!(ht.get(418), 32114);
        assert_eq!(ht.get(9), 514);
        assert_eq!(ht.get(27), 8917);
        assert_eq!(ht.get(1031), 286);
    }

    #[test]
    fn hash_is_key_mod_size() {
        assert_eq!(HashTable::hash(418, 193), 418 % 193);
        assert_eq!(HashTable::hash(9, 193), 9);
    }
}
