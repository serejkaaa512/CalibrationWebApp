use super::Unit;
use std::error::Error;

#[derive(Debug)]
pub struct PowerMeter {
    unit: Unit
}

impl PowerMeter {
    pub fn new(ip: & str, port: u16) -> PowerMeter {
        PowerMeter { unit: super::connect(ip, port).unwrap() }
    }
    
    pub fn get_power(&mut self, channel: u8) -> Result<f32, Box<Error>> {
        let get_power = &*format!("FETC{}?\n", channel.to_string());
        self.unit.get_query(get_power)
    }
}
