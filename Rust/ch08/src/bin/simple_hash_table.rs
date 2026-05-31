// 단순 해시 테이블 (Simple Hash Table)
// Clang/08/SimpleHashTable 포팅. 로직은 ch08::simple_hash_table 모듈에 있다.
use ch08::simple_hash_table::HashTable;

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
