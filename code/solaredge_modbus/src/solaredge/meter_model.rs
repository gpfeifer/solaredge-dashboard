use std::error::Error;

use modbus::{Client, Transport};



#[derive(Debug)]

/// Represents the Meter model for a device.
pub struct MeterModel {
	pub power: i16,
	pub power_scaling_factor: i16,
}

impl TryFrom<&mut Transport> for MeterModel {
	type Error = Box<dyn Error>;

	fn try_from(transport: &mut Transport) -> Result<Self, Self::Error> {
		// Reading holding registers
		let data = transport.read_holding_registers(40188, 23)?;
		// Convert the data to InverterModel
		let cm = MeterModel::from(data.as_slice());
		Ok(cm)
	}
}

impl From<&[u16]> for MeterModel {
	fn from(data: &[u16]) -> Self {
		let power = data[18] as i16;
		let power_scaling_factor = data[22] as i16;
		MeterModel {
			power,
			power_scaling_factor,
		}
	}
}
