use crate::error::Error;

/// Set a Rust string into a C string buffer, ensuring null-termination and checking for overflow.
pub fn set_c_string<const N: usize>(input: &str, target: &mut [i8; N]) -> Result<(), Error> {
    let bytes = input.as_bytes();

    if bytes.len() >= N {
        return Err(Error::BufferTooSmall {
            operation: "set_c_string",
            message: format!(
                "C string with length {} exceeds buffer size {}",
                bytes.len(),
                N
            )
            .into(),
            remaining: N,
            required: bytes.len() + 1,
        });
    }

    target.fill(0);
    let limit = bytes.len().min(N.saturating_sub(1));
    for (dest, &src) in target.iter_mut().zip(&input.as_bytes()[..limit]) {
        *dest = src as i8;
    }
    Ok(())
}

/// Convert a C string (null-terminated byte array) to a Rust `String`, stopping at the first null byte.
pub fn c_string_to_rust(c_string: &[i8]) -> String {
    let bytes = c_string
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as u8)
        .collect::<Vec<u8>>();
    String::from_utf8_lossy(&bytes).into_owned()
}

#[cfg(test)]
mod tests {
    use super::{c_string_to_rust, set_c_string};
    use crate::error::Error;

    #[test]
    fn set_c_string_accepts_max_payload_with_trailing_nul() {
        let mut target = [1_i8; 5];

        set_c_string("ABCD", &mut target).expect("input shorter than buffer should fit");

        assert_eq!(target, [b'A' as i8, b'B' as i8, b'C' as i8, b'D' as i8, 0]);
    }

    #[test]
    fn set_c_string_rejects_input_that_cannot_fit_nul() {
        let mut target = [0_i8; 4];

        let err = set_c_string("WXYZ", &mut target).expect_err("input length equal to N must fail");

        match err {
            Error::BufferTooSmall {
                operation,
                remaining,
                required,
                ..
            } => {
                assert_eq!(operation, "set_c_string");
                assert_eq!(remaining, 4);
                assert_eq!(required, 5);
            }
            other => panic!("unexpected error variant: {other:?}"),
        }
    }

    #[test]
    fn set_c_string_clears_previous_contents() {
        let mut target = [b'X' as i8, b'Y' as i8, b'Z' as i8, b'Q' as i8, b'R' as i8];

        set_c_string("A", &mut target).expect("single-byte input should fit");

        assert_eq!(target, [b'A' as i8, 0, 0, 0, 0]);
    }

    #[test]
    fn set_c_string_zero_length_buffer_always_errors() {
        let mut target = [0_i8; 0];

        let err = set_c_string("", &mut target).expect_err("no bytes can fit in zero-sized buffer");

        match err {
            Error::BufferTooSmall {
                operation,
                remaining,
                required,
                ..
            } => {
                assert_eq!(operation, "set_c_string");
                assert_eq!(remaining, 0);
                assert_eq!(required, 1);
            }
            other => panic!("unexpected error variant: {other:?}"),
        }
    }

    #[test]
    fn c_string_to_rust_stops_at_first_nul() {
        let input = [b'H' as i8, b'i' as i8, 0, b'X' as i8, b'Y' as i8];

        let output = c_string_to_rust(&input);

        assert_eq!(output, "Hi");
    }

    #[test]
    fn c_string_to_rust_returns_all_bytes_when_no_nul() {
        let input = [b'O' as i8, b'K' as i8];

        let output = c_string_to_rust(&input);

        assert_eq!(output, "OK");
    }

    #[test]
    fn c_string_to_rust_handles_non_utf8_bytes_lossily() {
        let input = [0x48_i8, -1_i8, 0];

        let output = c_string_to_rust(&input);

        assert_eq!(output, "H\u{fffd}");
    }

    #[test]
    fn c_string_to_rust_empty_input_is_empty_string() {
        let output = c_string_to_rust(&[]);

        assert_eq!(output, "");
    }
}
