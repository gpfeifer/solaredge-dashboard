use std::error::Error;

use modbus::{Client, Transport};

use super::convert::Convert;


#[derive(Debug)]
/// Represents the common model for a device.
pub struct CommonModel {
	/// The SunSpec ID of the device.
	pub sunspec_id: String,
	/// The manufacturer of the device.
	pub manufacturer: String,
	/// The model of the device.
	pub model: String,
	/// The version of the device.
	pub version: String,
	/// The serial number of the device.
	pub serial_number: String,
}

/// Converts a mutable reference to `Transport` into a `CommonModel` instance.
///
/// # Arguments
///
/// * `transport` - A mutable reference to the `Transport` object.
///
/// # Returns
///
/// Returns a `Result` containing the converted `CommonModel` instance on success, or an error
/// implementing the `Error` trait on failure.
impl TryFrom<&mut Transport> for CommonModel {
	type Error = Box<dyn Error>;

	fn try_from(transport: &mut Transport) -> Result<Self, Self::Error> {
		// Reading holding registers starting at address 40000 with quantity 70
		let data = transport.read_holding_registers(40000, 70)?;
		// Convert the data to CommonModel
		let cm = CommonModel::from(data.as_slice());
		Ok(cm)
	}
}

/// Converts a slice of u16 values into a `CommonModel` struct.
///
/// # Arguments
///
/// * `data` - The slice of u16 values to convert.
///
/// # Returns
///
/// A `CommonModel` struct containing the converted values.
impl From<&[u16]> for CommonModel {
	fn from(data: &[u16]) -> Self {
		let sunspec_id = data.to_string(0..2);
		let manufacturer = data.to_string(4..20);
		let model = data.to_string(20..36);
		let version = data.to_string(36..44);
		let serial_number = data.to_string(45..61);
		CommonModel {
			sunspec_id,
			manufacturer,
			model,
			version,
			serial_number,
		}
	}
}


