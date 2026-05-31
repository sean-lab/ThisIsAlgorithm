// 체이닝 해시 테이블 (Chaining)
// Clang/08/Chaining 포팅. 로직은 ch08::chaining 모듈에 있다.
use ch08::chaining::HashTable;

fn main() {
    let mut ht = HashTable::create(12289);

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
        println!("Key:{}, Value:{}", key, ht.get(key).unwrap());
    }
}
