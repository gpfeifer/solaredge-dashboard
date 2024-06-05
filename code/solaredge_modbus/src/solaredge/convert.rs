use std::ops::Range;

/// Trait for converting a slice of `u16` values to a different types.
pub trait Convert {
	/// Converts a range of `u16` values to a string.
	///
	/// # Arguments
	///
	/// * `range` - The range of indices to convert.
	///
	/// # Returns
	///
	/// The converted string.
	fn to_string(&self, range: Range<usize>) -> String;
}

/// Implementation of the `Convert` trait for slices of `u16` values.
impl Convert for &[u16] {
	/// Converts a range of `u16` values to a string.
	///
	/// # Arguments
	///
	/// * `range` - The range of indices to convert.
	///
	/// # Returns
	///
	/// The converted string.
	fn to_string(&self, range: Range<usize>) -> String {
		let mut bytes: Vec<u8> = vec![];
		for i in range {
			bytes.extend_from_slice(&self[i].to_be_bytes());
		}
		str_from_u8_nul_utf8_unchecked(&bytes)
	}
}

/// Converts a UTF-8 encoded byte slice to a string.
/// Only the bytes until null- terminator are converted.
/// If the slice is not null-terminated, the entire slice is converted.
///
/// # Arguments
///
/// * `bytes` - The UTF-8 encoded byte slice.
///
/// # Returns
///
/// The resulting string.
pub fn str_from_u8_nul_utf8_unchecked(bytes: &[u8]) -> String {
	let nul_range_end = bytes.iter()
		.position(|&c| c == b'\0')
		.unwrap_or(bytes.len()); // default to length if no `\0` present
	String::from_utf8_lossy(&bytes[0..nul_range_end]).to_string()
}

#[test]
fn test_convert() {
	let data: &[u16] = &[0x5375, 0x6e53, 0x0, 0x5357];
	assert_eq!(data.to_string(0..4), "SunS");
}