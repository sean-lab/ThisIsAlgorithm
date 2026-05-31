// 후위 표기식 계산기 (Calculator)
// Clang/02/Calculator 포팅. 중위표기 -> 후위표기 변환 후 계산.
use crate::linked_list_stack::LinkedListStack;

// SYMBOL enum (C): 문자 코드 그대로 사용. OPERAND = SPACE(' ')+1 = 33.
pub const LEFT_PARENTHESIS: u8 = b'(';
pub const RIGHT_PARENTHESIS: i32 = b')' as i32;
pub const SPACE: i32 = b' ' as i32;
pub const OPERAND: i32 = (b' ' as i32) + 1;

const NUMBER: &[u8] = b"0123456789.";

fn is_number(cipher: u8) -> bool {
    NUMBER.contains(&cipher)
}

// C의 GetNextToken: 토큰의 길이와 타입을 반환한다.
// 토큰 문자열은 expr[0..len] 이다 (C에서 Token[i]=Expression[i] 복사와 동일).
pub fn get_next_token(expr: &[u8]) -> (usize, i32) {
    let mut typ: i32 = -1;
    let mut i: usize = 0;

    loop {
        if i >= expr.len() || expr[i] == 0 {
            break;
        }

        if is_number(expr[i]) {
            typ = OPERAND;
            let next = if i + 1 < expr.len() { expr[i + 1] } else { 0 };
            if !is_number(next) {
                break;
            }
        } else {
            typ = expr[i] as i32;
            break;
        }
        i += 1;
    }

    (i + 1, typ)
}

pub fn get_priority(operator: u8, in_stack: bool) -> i32 {
    match operator {
        b'(' => {
            if in_stack {
                3
            } else {
                0
            }
        }
        b'*' | b'/' => 1,
        b'+' | b'-' => 2,
        _ => -1,
    }
}

pub fn is_prior(operator_in_stack: u8, operator_in_token: u8) -> bool {
    get_priority(operator_in_stack, true) > get_priority(operator_in_token, false)
}

pub fn get_postfix(infix: &str) -> String {
    let bytes = infix.as_bytes();
    let mut stack = LinkedListStack::new();
    let mut postfix = String::new();
    let mut position: usize = 0;
    let length = bytes.len();

    while position < length {
        let (len, typ) = get_next_token(&bytes[position..]);
        let token =
            String::from_utf8_lossy(&bytes[position..(position + len).min(length)]).into_owned();
        position += len;

        if typ == OPERAND {
            postfix.push_str(&token);
            postfix.push(' ');
        } else if typ == RIGHT_PARENTHESIS {
            while !stack.is_empty() {
                let popped = stack.pop().unwrap();
                if popped.as_bytes()[0] == LEFT_PARENTHESIS {
                    break;
                } else {
                    postfix.push_str(&popped);
                }
            }
        } else {
            while !stack.is_empty()
                && !is_prior(stack.top().unwrap().as_bytes()[0], token.as_bytes()[0])
            {
                let popped = stack.pop().unwrap();
                if popped.as_bytes()[0] != LEFT_PARENTHESIS {
                    postfix.push_str(&popped);
                }
            }
            stack.push(token);
        }
    }

    while !stack.is_empty() {
        let popped = stack.pop().unwrap();
        if popped.as_bytes()[0] != LEFT_PARENTHESIS {
            postfix.push_str(&popped);
        }
    }

    postfix
}

// C의 gcvt(value, 10, buffer): 유효숫자 10자리로 변환.
pub fn gcvt10(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if value.is_infinite() || value.is_nan() {
        return format!("{}", value);
    }
    let digits = 10i32;
    let exp = value.abs().log10().floor() as i32;
    let decimals = (digits - 1 - exp).max(0);
    let factor = 10f64.powi(decimals);
    let rounded = (value * factor).round() / factor;
    format!("{}", rounded)
}

pub fn calculate(postfix: &str) -> f64 {
    let bytes = postfix.as_bytes();
    let mut stack = LinkedListStack::new();
    let mut read: usize = 0;
    let length = bytes.len();

    while read < length {
        let (len, typ) = get_next_token(&bytes[read..]);
        let token =
            String::from_utf8_lossy(&bytes[read..(read + len).min(length)]).into_owned();
        read += len;

        if typ == SPACE {
            continue;
        }

        if typ == OPERAND {
            stack.push(token);
        } else {
            let operator2: f64 = stack.pop().unwrap().trim().parse().unwrap_or(0.0);
            let operator1: f64 = stack.pop().unwrap().trim().parse().unwrap_or(0.0);

            let temp_result = match typ as u8 {
                b'+' => operator1 + operator2,
                b'-' => operator1 - operator2,
                b'*' => operator1 * operator2,
                b'/' => operator1 / operator2,
                _ => 0.0,
            };

            stack.push(gcvt10(temp_result));
        }
    }

    stack.pop().unwrap().trim().parse().unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postfix_simple() {
        assert_eq!(get_postfix("3+4"), "3 4 +");
    }

    #[test]
    fn postfix_precedence() {
        // 곱셈이 덧셈보다 우선
        assert_eq!(get_postfix("3+4*2"), "3 4 2 *+");
    }

    #[test]
    fn postfix_parentheses() {
        assert_eq!(get_postfix("(3+4)*2"), "3 4 +2 *");
    }

    #[test]
    fn calculate_basic() {
        assert_eq!(calculate(&get_postfix("3+4")), 7.0);
        assert_eq!(calculate(&get_postfix("3+4*2")), 11.0);
        assert_eq!(calculate(&get_postfix("(3+4)*2")), 14.0);
    }

    #[test]
    fn calculate_division() {
        assert_eq!(calculate(&get_postfix("10/4")), 2.5);
    }

    #[test]
    fn priority_rules() {
        // get_priority: 숫자가 작을수록 연산 우선순위가 높다 (* /=1, + -=2).
        assert_eq!(get_priority(b'*', false), 1);
        assert_eq!(get_priority(b'+', false), 2);
        // is_prior는 스택 우선순위 숫자 > 토큰 우선순위 숫자일 때 true.
        assert!(is_prior(b'+', b'*')); // 2 > 1
        assert!(!is_prior(b'*', b'+')); // 1 > 2 아님
    }

    #[test]
    fn gcvt10_trims_trailing() {
        assert_eq!(gcvt10(7.0), "7");
        assert_eq!(gcvt10(2.5), "2.5");
        assert_eq!(gcvt10(0.0), "0");
    }
}
