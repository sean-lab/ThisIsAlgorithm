// 개방 주소법 해시 테이블 (Open Addressing, 이중 해싱)
// Clang/08/OpenAddressing 포팅.
pub const EMPTY: i32 = 0;
pub const OCCUPIED: i32 = 1;

#[derive(Clone, Default)]
pub struct Element {
    pub key: String,
    pub value: String,
    pub status: i32,
}

pub struct HashTable {
    pub occupied_count: i32,
    pub table_size: i32,
    pub table: Vec<Element>,
}

impl HashTable {
    pub fn create(table_size: i32) -> HashTable {
        HashTable {
            occupied_count: 0,
            table_size,
            table: vec![Element::default(); table_size as usize],
        }
    }

    pub fn hash(key: &str, table_size: i32) -> i32 {
        let mut hash_value: i32 = 0;
        for &b in key.as_bytes() {
            hash_value = hash_value.wrapping_shl(3).wrapping_add(b as i32);
        }
        hash_value.rem_euclid(table_size)
    }

    pub fn hash2(key: &str, table_size: i32) -> i32 {
        let mut hash_value: i32 = 0;
        for &b in key.as_bytes() {
            hash_value = hash_value.wrapping_shl(2).wrapping_add(b as i32);
        }
        hash_value.rem_euclid(table_size - 3) + 1
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let usage = self.occupied_count as f64 / self.table_size as f64;

        if usage > 0.5 {
            self.rehash();
        }

        let mut address = HashTable::hash(key, self.table_size);
        let step_size = HashTable::hash2(key, self.table_size);

        while self.table[address as usize].status != EMPTY
            && self.table[address as usize].key != key
        {
            println!(
                "Collision occured! : Key({}), Address({}), StepSize({})",
                key, address, step_size
            );

            address = (address + step_size) % self.table_size;
        }

        self.table[address as usize].key = key.to_string();
        self.table[address as usize].value = value.to_string();
        self.table[address as usize].status = OCCUPIED;

        self.occupied_count += 1;

        println!("Key({}) entered at address({})", key, address);
    }

    pub fn get(&self, key: &str) -> &str {
        let mut address = HashTable::hash(key, self.table_size);
        let step_size = HashTable::hash2(key, self.table_size);

        while self.table[address as usize].status != EMPTY
            && self.table[address as usize].key != key
        {
            address = (address + step_size) % self.table_size;
        }

        &self.table[address as usize].value
    }

    pub fn rehash(&mut self) {
        let new_size = self.table_size * 2;
        let mut new_ht = HashTable::create(new_size);

        println!("\nRehashed. New table size is : {}\n", new_ht.table_size);

        let old: Vec<(String, String)> = self
            .table
            .iter()
            .filter(|e| e.status == OCCUPIED)
            .map(|e| (e.key.clone(), e.value.clone()))
            .collect();

        for (k, v) in old {
            new_ht.set(&k, &v);
        }

        *self = new_ht;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn populate() -> HashTable {
        let mut ht = HashTable::create(11);
        ht.set("MSFT", "Microsoft Corporation");
        ht.set("JAVA", "Sun Microsystems");
        ht.set("REDH", "Red Hat Linux");
        ht.set("APAC", "Apache Org");
        ht.set("ZYMZZ", "Unisys Ops Check");
        ht.set("IBM", "IBM Ltd.");
        ht.set("ORCL", "Oracle Corporation");
        ht.set("CSCO", "Cisco Systems, Inc.");
        ht
    }

    #[test]
    fn get_returns_stored_value_after_rehash() {
        // 적재율이 0.5를 넘으면 rehash 가 발생한다. 그래도 값은 조회 가능해야 한다.
        let ht = populate();
        assert!(ht.table_size > 11);
        assert_eq!(ht.get("MSFT"), "Microsoft Corporation");
        assert_eq!(ht.get("APAC"), "Apache Org");
        assert_eq!(ht.get("CSCO"), "Cisco Systems, Inc.");
    }

    #[test]
    fn hash2_is_in_valid_range() {
        let size = 11;
        let s = HashTable::hash2("APAC", size);
        assert!((1..=size - 3 + 1).contains(&s));
    }
}
