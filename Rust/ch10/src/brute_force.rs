// 무차별 대입 문자열 탐색 (Brute Force)
// Clang/10/BruteForce 포팅.
pub fn brute_force(
    text: &[u8],
    text_size: i32,
    start: i32,
    pattern: &[u8],
    pattern_size: i32,
) -> i32 {
    let mut i = start;
    while i <= text_size - pattern_size {
        let mut j = 0;
        while j < pattern_size {
            if text[(i + j) as usize] != pattern[j as usize] {
                break;
            }
            j += 1;
        }
        if j >= pattern_size {
            return i;
        }
        i += 1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find(text: &str, pattern: &str) -> i32 {
        brute_force(
            text.as_bytes(),
            text.len() as i32,
            0,
            pattern.as_bytes(),
            pattern.len() as i32,
        )
    }

    #[test]
    fn finds_pattern_position() {
        assert_eq!(find("hello world", "world"), 6);
        assert_eq!(find("abcabcabd", "abcabd"), 3);
        assert_eq!(find("aaaaa", "aaa"), 0);
    }

    #[test]
    fn returns_minus_one_when_absent() {
        assert_eq!(find("hello", "xyz"), -1);
    }
}
