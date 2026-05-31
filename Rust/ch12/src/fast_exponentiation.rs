// 빠른 거듭제곱 (Fast Exponentiation)
// Clang/12/FastExponentiation 포팅.

pub fn power(base: i32, exponent: i32) -> u64 {
    if exponent == 1 {
        return base as u64;
    } else if base == 0 {
        return 1;
    }

    if exponent % 2 == 0 {
        let new_base = power(base, exponent / 2);
        new_base.wrapping_mul(new_base)
    } else {
        let new_base = power(base, (exponent - 1) / 2);
        new_base.wrapping_mul(new_base).wrapping_mul(base as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_powers() {
        assert_eq!(power(2, 1), 2);
        assert_eq!(power(2, 10), 1024);
        assert_eq!(power(3, 4), 81);
    }

    #[test]
    fn base_zero_returns_one() {
        // base == 0 인 경우 본문 로직은 1을 반환한다(exponent != 1 일 때).
        assert_eq!(power(0, 5), 1);
    }

    #[test]
    fn matches_main_program() {
        assert_eq!(power(2, 30), 1073741824);
    }
}
