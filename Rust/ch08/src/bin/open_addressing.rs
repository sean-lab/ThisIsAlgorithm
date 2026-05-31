// 개방 주소법 해시 테이블 (Open Addressing, 이중 해싱)
// Clang/08/OpenAddressing 포팅.
const EMPTY: i32 = 0;
const OCCUPIED: i32 = 1;

#[derive(Clone, Default)]
struct Element {
    key: String,
    value: String,
    status: i32,
}

struct HashTable {
    occupied_count: i32,
    table_size: i32,
    table: Vec<Element>,
}

impl HashTable {
    fn create(table_size: i32) -> HashTable {
        HashTable {
            occupied_count: 0,
            table_size,
            table: vec![Element::default(); table_size as usize],
        }
    }

    fn hash(key: &str, table_size: i32) -> i32 {
        let mut hash_value: i32 = 0;
        for &b in key.as_bytes() {
            hash_value = hash_value.wrapping_shl(3).wrapping_add(b as i32);
        }
        hash_value.rem_euclid(table_size)
    }

    fn hash2(key: &str, table_size: i32) -> i32 {
        let mut hash_value: i32 = 0;
        for &b in key.as_bytes() {
            hash_value = hash_value.wrapping_shl(2).wrapping_add(b as i32);
        }
        hash_value.rem_euclid(table_size - 3) + 1
    }

    fn set(&mut self, key: &str, value: &str) {
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

    fn get(&self, key: &str) -> &str {
        let mut address = HashTable::hash(key, self.table_size);
        let step_size = HashTable::hash2(key, self.table_size);

        while self.table[address as usize].status != EMPTY
            && self.table[address as usize].key != key
        {
            address = (address + step_size) % self.table_size;
        }

        &self.table[address as usize].value
    }

    fn rehash(&mut self) {
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

fn main() {
    let mut ht = HashTable::create(11);

    ht.set("MSFT", "Microsoft Corporation");
    ht.set("JAVA", "Sun Microsystems");
    ht.set("REDH", "Red Hat Linux");
    ht.set("APAC", "Apache Org");
    ht.set("ZYMZZ", "Unisys Ops Check"); // APAC와 충돌
    ht.set("IBM", "IBM Ltd.");
    ht.set("ORCL", "Oracle Corporation");
    ht.set("CSCO", "Cisco Systems, Inc.");
    ht.set("GOOG", "Google Inc.");
    ht.set("YHOO", "Yahoo! Inc.");
    ht.set("NOVL", "Novell, Inc.");

    println!();
    for key in [
        "MSFT", "REDH", "APAC", "ZYMZZ", "JAVA", "IBM", "ORCL", "CSCO", "GOOG", "YHOO", "NOVL",
    ] {
        println!("Key:{}, Value:{}", key, ht.get(key));
    }
}
