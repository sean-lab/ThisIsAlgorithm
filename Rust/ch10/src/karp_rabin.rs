// 카프-라빈 (Karp-Rabin)
// Clang/10/KarpRabin 포팅. C char 는 signed 이므로 i8 로 변환해 충실히 포팅.
pub fn hash(string: &[u8], size: i32) -> i32 {
    let mut hash_value: i32 = 0;
    let mut i = 0;
    while i < size {
        // C `char` is signed on x86/arm.
        hash_value = (string[i as usize] as i8 as i32).wrapping_add(hash_value.wrapping_mul(2));
        i += 1;
    }
    hash_value
}

pub fn re_hash(string: &[u8], start: i32, size: i32, hash_prev: i32, coefficient: i32) -> i32 {
    if start == 0 {
        return hash_prev;
    }
    let last = string[(start + size - 1) as usize] as i8 as i32;
    let first = string[(start - 1) as usize] as i8 as i32;
    last.wrapping_add(
        hash_prev
            .wrapping_sub(coefficient.wrapping_mul(first))
            .wrapping_mul(2),
    )
}

pub fn karp_rabin(
    text: &[u8],
    text_size: i32,
    start: i32,
    pattern: &[u8],
    pattern_size: i32,
) -> i32 {
    let coefficient = 2f64.powi(pattern_size - 1) as i32;
    let mut hash_text = hash(text, pattern_size);
    let hash_pattern = hash(pattern, pattern_size);

    let mut i = start;
    while i <= text_size - pattern_size {
        hash_text = re_hash(text, i, pattern_size, hash_text, coefficient);

        if hash_pattern == hash_text {
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
        }
        i += 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find(text: &str, pattern: &str) -> i32 {
        karp_rabin(
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

    #[test]
    fn equal_strings_have_equal_hash() {
        let a = b"abcd";
        assert_eq!(hash(a, 4), hash(b"abcd", 4));
    }
}
