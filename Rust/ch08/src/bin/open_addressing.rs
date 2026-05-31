// 개방 주소법 해시 테이블 (Open Addressing, 이중 해싱)
// Clang/08/OpenAddressing 포팅. 로직은 ch08::open_addressing 모듈에 있다.
use ch08::open_addressing::HashTable;

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
