// 후위 표기식 계산기 (Calculator)
// Clang/02/Calculator 포팅. 로직은 ch02::calculator 모듈에 있다.
use ch02::calculator::{calculate, get_postfix};
use std::io::{self, Read, Write};

fn main() {
    print!("Enter Infix Expression:");
    io::stdout().flush().ok();

    // scanf("%99s", ...) 와 동일하게 공백 전까지의 토큰 하나를 읽는다.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let infix_expression: String = input.split_whitespace().next().unwrap_or("").to_string();

    let postfix_expression = get_postfix(&infix_expression);

    println!("Infix:{}\nPostfix:{}", infix_expression, postfix_expression);

    let result = calculate(&postfix_expression);

    println!("Calculation Result : {:.6}", result);
}
