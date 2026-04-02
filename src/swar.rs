//! SWAR (SIMD Within A Register) optimization for JSON string scanning.
//!
//! Checks 8 bytes at a time using bitwise operations on a `u64` to find
//! bytes that require special handling in JSON strings: control characters
//! (< 0x20), double quote (0x22), and backslash (0x5C).

const MASK_80: u64 = 0x8080808080808080;
const MASK_01: u64 = 0x0101010101010101;

/// Returns the number of leading bytes in `s` that are "plain" ASCII for JSON strings:
/// - Not a control character (byte >= 0x20)
/// - Not a double quote `"` (0x22)
/// - Not a backslash `\` (0x5C)
/// - ASCII (byte < 0x80), so byte/char boundaries are guaranteed to align
pub(crate) fn skip_plain_ascii_bytes(s: &[u8]) -> usize {
    let mut i = 0;

    // Process 8 bytes at a time
    while i + 8 <= s.len() {
        let chunk: [u8; 8] = s[i..i + 8].try_into().unwrap();
        let w = u64::from_ne_bytes(chunk);

        if is_all_plain_ascii(w) {
            i += 8;
        } else {
            // Find the first non-plain byte within this chunk
            return i + first_non_plain_offset(w);
        }
    }

    // Handle remaining bytes (< 8)
    while i < s.len() {
        if is_plain_ascii_byte(s[i]) {
            i += 1;
        } else {
            break;
        }
    }

    i
}

/// Returns the number of leading bytes in `s` that belong to a non-ASCII UTF-8 run.
///
/// This assumes `s` starts at a UTF-8 character boundary. It stops at the first ASCII byte,
/// which is also a character boundary in valid UTF-8.
#[inline(always)]
pub(crate) fn skip_non_ascii_bytes(s: &[u8]) -> usize {
    let mut i = 0;

    // Process 8 bytes at a time: all non-ASCII bytes have bit 7 set
    while i + 8 <= s.len() {
        let chunk: [u8; 8] = s[i..i + 8].try_into().unwrap();
        let w = u64::from_ne_bytes(chunk);
        if w & MASK_80 == MASK_80 {
            i += 8;
        } else {
            break;
        }
    }

    while i < s.len() && s[i] & 0x80 != 0 {
        i += 1;
    }
    i
}

/// Build a mask with bit 7 set for each byte in `w` that is NOT plain ASCII for JSON strings.
/// A byte is non-plain if: >= 128, < 32, == 0x22 (`"`), or == 0x5C (`\`).
#[inline(always)]
fn non_plain_mask(w: u64) -> u64 {
    // Non-ASCII: byte >= 128
    let non_ascii = w & MASK_80;

    // Control chars: byte < 32 (for bytes < 128)
    // For b in [0x00, 0x7F]: b + 0x60 sets bit 7 iff b >= 0x20
    let control = (w.wrapping_add(0x6060606060606060) ^ MASK_80) & MASK_80;

    // Quote (0x22): Mycroft's zero-byte detection
    let xor_quote = w ^ 0x2222222222222222;
    let quote = xor_quote.wrapping_sub(MASK_01) & !xor_quote & MASK_80;

    // Backslash (0x5C): Mycroft's zero-byte detection
    let xor_bslash = w ^ 0x5C5C5C5C5C5C5C5C;
    let bslash = xor_bslash.wrapping_sub(MASK_01) & !xor_bslash & MASK_80;

    non_ascii | control | quote | bslash
}

/// Check if all 8 bytes in `w` are plain ASCII for JSON strings.
#[inline(always)]
fn is_all_plain_ascii(w: u64) -> bool {
    non_plain_mask(w) == 0
}

/// Find the byte offset of the first non-plain byte within a u64 word.
/// Assumes at least one non-plain byte exists.
#[inline(always)]
fn first_non_plain_offset(w: u64) -> usize {
    let fail = non_plain_mask(w);

    // Find the first set bit. On little-endian, trailing_zeros gives the
    // lowest-address byte. On big-endian, leading_zeros does.
    #[cfg(target_endian = "little")]
    {
        (fail.trailing_zeros() / 8) as usize
    }
    #[cfg(target_endian = "big")]
    {
        (fail.leading_zeros() / 8) as usize
    }
}

#[inline(always)]
fn is_plain_ascii_byte(b: u8) -> bool {
    (0x20..0x80).contains(&b) && b != b'"' && b != b'\\'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(skip_plain_ascii_bytes(b""), 0);
    }

    #[test]
    fn all_plain_short() {
        assert_eq!(skip_plain_ascii_bytes(b"hello"), 5);
    }

    #[test]
    fn all_plain_8_bytes() {
        assert_eq!(skip_plain_ascii_bytes(b"abcdefgh"), 8);
    }

    #[test]
    fn all_plain_16_bytes() {
        assert_eq!(skip_plain_ascii_bytes(b"abcdefghijklmnop"), 16);
    }

    #[test]
    fn quote_at_various_positions() {
        for pos in 0..16 {
            let mut buf = [b'a'; 16];
            buf[pos] = b'"';
            assert_eq!(
                skip_plain_ascii_bytes(&buf),
                pos,
                "quote at position {}",
                pos
            );
        }
    }

    #[test]
    fn backslash_at_various_positions() {
        for pos in 0..16 {
            let mut buf = [b'a'; 16];
            buf[pos] = b'\\';
            assert_eq!(
                skip_plain_ascii_bytes(&buf),
                pos,
                "backslash at position {}",
                pos
            );
        }
    }

    #[test]
    fn control_chars() {
        for b in 0..0x20u8 {
            let buf = [b'a', b'b', b'c', b];
            assert_eq!(skip_plain_ascii_bytes(&buf), 3, "control char 0x{:02x}", b);
        }
    }

    #[test]
    fn control_char_at_start() {
        assert_eq!(skip_plain_ascii_bytes(b"\x00abc"), 0);
        assert_eq!(skip_plain_ascii_bytes(b"\nabc"), 0);
        assert_eq!(skip_plain_ascii_bytes(b"\tabc"), 0);
    }

    #[test]
    fn non_ascii_stops() {
        assert_eq!(skip_plain_ascii_bytes("abc日本語".as_bytes()), 3);
        assert_eq!(skip_plain_ascii_bytes("あ".as_bytes()), 0);
    }

    #[test]
    fn non_ascii_run() {
        assert_eq!(skip_non_ascii_bytes("日本語x".as_bytes()), "日本語".len());
        assert_eq!(skip_non_ascii_bytes("あa".as_bytes()), "あ".len());
        assert_eq!(skip_non_ascii_bytes("abc".as_bytes()), 0);
    }

    #[test]
    fn high_ascii_boundary() {
        // 0x7F (DEL) is a control char in JSON context? No, JSON spec says < 0x20 are control.
        // DEL (0x7F) is technically a control char but JSON spec only requires escaping < 0x20.
        // Our function treats 0x7F as plain ASCII (>= 0x20 and < 0x80).
        assert_eq!(skip_plain_ascii_bytes(&[0x7F]), 1);
        // 0x80 is non-ASCII
        assert_eq!(skip_plain_ascii_bytes(&[0x80]), 0);
        // 0x20 (space) is the first plain byte
        assert_eq!(skip_plain_ascii_bytes(&[0x20]), 1);
        // 0x1F is last control char
        assert_eq!(skip_plain_ascii_bytes(&[0x1F]), 0);
    }

    #[test]
    fn mixed_content() {
        let input = b"Hello, World!\"rest";
        assert_eq!(skip_plain_ascii_bytes(input), 13); // stops at "
    }

    #[test]
    fn all_printable_ascii() {
        // All printable ASCII except " and \ should be plain
        let mut count = 0;
        for b in 0x20..0x80u8 {
            if b != b'"' && b != b'\\' {
                assert_eq!(
                    skip_plain_ascii_bytes(&[b]),
                    1,
                    "byte 0x{:02x} ('{}') should be plain",
                    b,
                    b as char,
                );
                count += 1;
            }
        }
        assert_eq!(count, 94); // 96 printable minus " and \
    }

    #[test]
    fn long_plain_then_special() {
        let buf = [b'x'; 1024];
        // All 1024 bytes are plain 'x'
        assert_eq!(skip_plain_ascii_bytes(&buf), 1024);

        let mut buf2 = [b'x'; 64];
        buf2[50] = b'"';
        assert_eq!(skip_plain_ascii_bytes(&buf2), 50);
    }
}
