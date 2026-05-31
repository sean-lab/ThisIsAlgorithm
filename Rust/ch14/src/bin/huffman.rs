// Huffman coding - 로직은 ch14::huffman 모듈에 있다.
use ch14::huffman::run;

fn main() {
    let source: &[u8] = b"This Is Algorithms.\0";

    let result = run(source);

    println!(
        "Original Size:{} Encoded Size:{}",
        result.original_size, result.encoded_size
    );

    print!("Original : ");
    println!("{}", result.original);
    print!("Encoded  : ");
    println!("{}", result.binary);
    println!("Decoded  : {}", result.decoded);
}
