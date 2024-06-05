use std::net::SocketAddr;

use log::{debug, error, info};

use telegraf::{Metric, TelegrafResult};


mod solaredge;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adr = "solaredge";
    let cfg = modbus::tcp::Config{ tcp_port: 1520, ..Default::default() };
    // config.tcp_port = 1520;
    // let config = modbus::tcp::Config{ 
    //     tcp_port: 1520,
    //     tcp_connect_timeout: None, // Some(std::time::Duration::from_millis(5000)),
    //     tcp_read_timeout: None,
    //     tcp_write_timeout: None,
    //     modbus_uid: 1,    
    // };
    // std::thread::sleep(std::time::Duration::from_millis(1000));


    env_logger::init();
    // let addr = std::env::var("SOLAREDGE_ADDRESS").unwrap_or_else(|_|{
    //     println!("ERROR: Please set the SOLAREDGE_ADDRESS environment variable");
    //     std::process::exit(1);
    // });
    // let addr: SocketAddr = addr.parse().unwrap_or_else(|e|{
    //     println!("ERROR: Invalid address {}: {}", addr, e);
    //     std::process::exit(1);
    // });
    // let addr = "solaredge";
    let addr = "192.168.178.96";
    let port = 1502;
    println!("Connecting to SolarEdge inverter at {}", addr);
    let mut solaredge = solaredge::SolarEdge::new(addr, port)?;

    println!("Connecting to Telegraf");
    let mut client = telegraf::Client::new("tcp://localhost:8094").unwrap_or_else(|e| {
        error!("ERROR: Connecting to Telegraf: {}", e);
        std::process::exit(1);
    });

    let mut total = 0;
    let mut failed = 0;
    let mut reconnect: u32 = 0;
    let mut max: u32 = 0;
    loop {
        // let data = solaredge.read_data();
        total += 1;
        match solaredge.read_data() {
            Some(data) => {
                send_to_telegraf(&mut client, &data, reconnect).unwrap_or_else(|e| {
                    error!("ERROR: Sending to Telegraf: {}", e);
                    
                });
                reconnect = 0;
            }
            None => {
                reconnect += 1;
                if reconnect > max {
                    max = reconnect;
                }
                failed += 1;
                let p = 100.0 * (failed as f64) / (total as f64);
                error!("ERROR: read data {reconnect} times, {failed}/{total} {p:.2}% ");
                println!("ERROR: read data {reconnect} times, {failed}/{total} {p:.2}%, max: {max} ");
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    // let inverter = InverterModel::try_from(&mut ctx)?;
    // let meter = MeterModel::try_from(&mut ctx)?;
    // println!("Common: {:?}", common);
    // println!("Inverter: {:?}", inverter);
    // println!("Meter: {:?}", meter);
   // Ok(())
}

 
#[derive(Metric, Debug)]
struct SEMetric {
    se_produces: i16,
    se_consumed: i16,
    se_balance: i16,
    se_reconnect: u32,
    #[telegraf(tag)]
    influxdb_database: String,
}

fn send_to_telegraf(client: &mut telegraf::Client, data: &solaredge::SolarEdgeData, reconnect: u32) -> TelegrafResult {
    let metric = SEMetric {
        se_produces: data.produced,
        se_consumed: data.consumed,
        se_balance: data.balance,
        se_reconnect: reconnect,
        influxdb_database: "solaredge".to_string(),
    };
    debug!("Sending metric: {:?}", metric);
    client.write(&metric)
}



