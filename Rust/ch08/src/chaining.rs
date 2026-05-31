// 체이닝 해시 테이블 (Chaining)
// Clang/08/Chaining 포팅. 각 슬롯이 연결 리스트(헤드 삽입)인 안전한 구현.

pub struct Node {
    pub key: String,
    pub value: String,
    pub next: Option<Box<Node>>,
}

pub struct HashTable {
    pub table_size: usize,
    pub table: Vec<Option<Box<Node>>>,
}

impl HashTable {
    pub fn create(table_size: usize) -> HashTable {
        let mut table = Vec::with_capacity(table_size);
        for _ in 0..table_size {
            table.push(None);
        }
        HashTable { table_size, table }
    }

    pub fn hash(key: &str, table_size: usize) -> usize {
        // 원본 C: HashValue = (HashValue << 3) + Key[i]; (int 오버플로 wrapping)
        let mut hash_value: i32 = 0;
        for &b in key.as_bytes() {
            hash_value = hash_value.wrapping_shl(3).wrapping_add(b as i32);
        }
        (hash_value.rem_euclid(table_size as i32)) as usize
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let address = HashTable::hash(key, self.table_size);

        let new_node = Box::new(Node {
            key: key.to_string(),
            value: value.to_string(),
            next: None,
        });

        if self.table[address].is_none() {
            self.table[address] = Some(new_node);
        } else {
            let mut new_node = new_node;
            new_node.next = self.table[address].take();
            self.table[address] = Some(new_node);

            println!("Collision occured : Key({}), Address({})", key, address);
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        let address = HashTable::hash(key, self.table_size);

        let mut the_list = self.table[address].as_deref();
        let mut target: Option<&Node> = None;

        the_list?;

        loop {
            let node = the_list.unwrap();
            if node.key == key {
                target = Some(node);
                break;
            }

            if node.next.is_none() {
                break;
            } else {
                the_list = node.next.as_deref();
            }
        }

        target.map(|n| n.value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn populate() -> HashTable {
        let mut ht = HashTable::create(12289);
        ht.set("MSFT", "Microsoft Corporation");
        ht.set("JAVA", "Sun Microsystems");
        ht.set("REDH", "Red Hat Linux");
        ht.set("APAC", "Apache Org");
        ht.set("ZYMZZ", "Unisys Ops Check"); // APAC와 충돌
        ht.set("IBM", "IBM Ltd.");
        ht
    }

    #[test]
    fn get_returns_stored_value() {
        let ht = populate();
        assert_eq!(ht.get("MSFT"), Some("Microsoft Corporation"));
        assert_eq!(ht.get("APAC"), Some("Apache Org"));
        assert_eq!(ht.get("IBM"), Some("IBM Ltd."));
    }

    #[test]
    fn collision_keys_both_retrievable() {
        // ZYMZZ와 APAC은 같은 주소에 충돌하지만 둘 다 조회 가능해야 한다.
        let ht = populate();
        assert_eq!(
            HashTable::hash("ZYMZZ", 12289),
            HashTable::hash("APAC", 12289)
        );
        assert_eq!(ht.get("ZYMZZ"), Some("Unisys Ops Check"));
        assert_eq!(ht.get("APAC"), Some("Apache Org"));
    }

    #[test]
    fn missing_key_returns_none() {
        let ht = populate();
        assert_eq!(ht.get("NOPE"), None);
    }
}
