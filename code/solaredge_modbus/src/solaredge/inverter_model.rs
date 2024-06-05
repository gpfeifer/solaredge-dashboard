use std::error::Error;

use modbus::{Client, Transport};



#[derive(Debug)]

/// Represents the inverter model for a device.
pub struct InverterModel {
	pub power: i16,
	pub power_scaling_factor: i16,
}

impl TryFrom<&mut Transport> for InverterModel {
	type Error = Box<dyn Error>;

	fn try_from(transport: &mut Transport) -> Result<Self, Self::Error> {
		// Reading holding registers
		let data = transport.read_holding_registers(40069, 20)?;
		// Convert the data to InverterModel
		let cm = InverterModel::from(data.as_slice());
		Ok(cm)
	}
}

impl From<&[u16]> for InverterModel {
	fn from(data: &[u16]) -> Self {
		let power = data[14] as i16;
		let power_scaling_factor = data[15] as i16;
		InverterModel {
			power,
			power_scaling_factor,
		}
	}
}
