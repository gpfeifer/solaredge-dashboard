use std::net::SocketAddr;

use log::error;
use modbus::{tcp, Client, Config, Transport};


use self::{inverter_model::InverterModel, meter_model::MeterModel};

mod convert;
use convert::Convert;
mod common_model;
mod inverter_model;
mod meter_model;



/// Represents the data retrieved from a SolarEdge device.
#[derive(Debug)]
pub struct SolarEdgeData {
	/// The amount of energy produced by the SolarEdge device.
	pub produced: i16,
	/// The amount of energy consumed by the house.
	pub consumed: i16,
	/// The balance of the house (positive: sell power, negative: buy power ).
	pub balance: i16,

	// pub max_reconnect: usize,
}

pub struct SolarEdge {
	transport: Option<Transport>,
	adr: String,
	port: u16,
	max_reconnect: usize,
	failed_reconnect: usize,
}

impl SolarEdge {
	
	pub fn from(socket_addr: SocketAddr) -> std::io::Result<Self> {
		SolarEdge::new(&socket_addr.ip().to_string(), socket_addr.port())
	}

	pub fn new(adr: &str, port: u16) -> std::io::Result<Self> {
		let transport = SolarEdge::create_transport(adr, port)?;
		
		Ok(SolarEdge { 
			transport: Some(transport), 
			adr: adr.to_string(), 
			port,
			max_reconnect: 0,
			failed_reconnect: 0,
		})
	}
	
	fn create_transport(adr: &str, port: u16) -> std::io::Result<Transport>  {
		let config = Config{ 
			tcp_port: port,
			// tcp_read_timeout: Some(std::time::Duration::from_millis(5000)),
			// tcp_write_timeout: Some(std::time::Duration::from_millis(5000)),
			..Default::default()};
		println!("Connection to Solaredge at {adr}:{}", config.tcp_port);
		let transport = tcp::Transport::new_with_cfg(adr, config)?;
		Ok(transport)
	}
		
	pub fn read_data(&mut self) -> Option<SolarEdgeData> {

		match self.transport.as_mut() {
			Some(transport) => {
				SolarEdge::try_read_data(transport)
					.map_err(|e| {
						self.transport = None;
						e})
					.ok()
					
			}
			None => {
				self.failed_reconnect += 1;
				if self.failed_reconnect > self.max_reconnect {
					self.max_reconnect = self.failed_reconnect;
				}

				error!("Sleep {} seconds before trying to reconnect", self.failed_reconnect);
				std::thread::sleep(std::time::Duration::from_secs(self.failed_reconnect as u64));

				self.transport = SolarEdge::create_transport(&self.adr, self.port).ok();
				self.transport.as_mut().and_then(|transport| {
					SolarEdge::try_read_data(transport)
						.map(|data| {
							// Reset failed reconnect counter
							self.failed_reconnect = 0;
							data
						})
						.ok()
				})
			}
		}
	}
	
	fn try_read_data(transport: &mut Transport) -> Result<SolarEdgeData, Box<dyn std::error::Error>> {
		let inverter = read_inverter(transport)?;
		let meter = read_meter(transport)?;

		let produced = scale(inverter.power, inverter.power_scaling_factor);
		let balance = scale(meter.power, meter.power_scaling_factor);
		let consumed = produced - balance;

		Ok(SolarEdgeData {
			produced,
			consumed,
			balance,
			
		})
	}
	
	// pub(crate) fn run(&mut self) { 
	// 	let mut total: u32 = 0;
	// 	let mut failed: u32 = 0;
	// 	loop {
	// 		// let data = solaredge.read_data();
	// 		total += 1;
	// 		match self.read_data() {
	// 			Some(data) => {
	// 				println!("{:?}", data);
	// 			}
	// 			None => {
	// 				failed += 1;
	// 				error!("read_data() failed");
	// 			}
	// 		}
	// 		let p = 100.0 * (failed as f64) / (total as f64);
	// 		println!( "{failed}/{total} {p:.2}%"); 
	// 		std::thread::sleep(std::time::Duration::from_millis(1000));
	// 	}
		
	// }
}	

fn scale(value: i16, scaling_factor: i16) -> i16 {
	if scaling_factor >= 0 {
		value 
	} else {
		(value as f64 * f64::powi(10_f64, scaling_factor as i32)) as i16
	}
}

fn read_inverter(transport: &mut Transport) -> Result<InverterModel, Box<dyn std::error::Error>> {
	let inverter = InverterModel::try_from(transport)?;
	Ok(inverter)
}

fn read_meter(transport: &mut Transport) -> Result<MeterModel, Box<dyn std::error::Error>> {
	let meter = MeterModel::try_from(transport)?;
	Ok(meter)
}