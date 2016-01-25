use super::Unit;
use std::error::Error;


#[derive(Debug)]
pub struct Generator {
    unit: Unit
}

impl Generator {
    pub fn new(ip: & str, port: u16) -> Generator {
        Generator { unit: super::connect(ip, port).unwrap() }
    }

    pub fn get_freq(&mut self) -> Result<f32, Box<Error>> {
        self.unit.get_query("SOUR:FREQ:FIX?\n")
    }

    pub fn get_power(&mut self) -> Result<f32, Box<Error>> {
        self.unit.get_query("SOUR:POW:LEV:IMM:AMPL?\n")
    }

    pub fn set_freq(&mut self, freq: f32) -> Result<f32, Box<Error>> {
        let set_freq_str = &*format!("SOUR:FREQ:FIX {}\n", freq.to_string());
        let get_freq_str = &*format!("SOUR:FREQ:FIX?\n");
        self.unit.set_query(freq, set_freq_str, get_freq_str)
    }

    pub fn set_power(&mut self, power: f32) -> Result<f32, Box<Error>> {
        let set_power_str = &*format!("SOUR:POW:LEV:IMM:AMPL {}\n", power.to_string());
        let get_power_str = &*format!("SOUR:POW:LEV:IMM:AMPL?\n");
        self.unit.set_query(power, set_power_str, get_power_str)
    }

    pub fn set_power_on(&mut self) -> Option<Box<Error>> {
        let power = 1u16;
        let set_power_str = &*format!("OUTP:STAT {}\n", power);
        let get_power_str = &*format!("OUTP:STAT?\n");
        self.unit.set_query(power, set_power_str, get_power_str).err()
    }

    pub fn set_power_off(&mut self) -> Option<Box<Error>> {
        let power = 0u16;
        let set_power_str = &*format!("OUTP:STAT {}\n", power);
        let get_power_str = &*format!("OUTP:STAT?\n");
        self.unit.set_query(power, set_power_str, get_power_str).err()
    }

    pub fn get_power_on(&mut self) -> Result<bool, Box<Error>> {
        let get_power_on_str = &*format!("OUTP:STAT?\n");
        let power_on: u16 = try!(self.unit.get_query(get_power_on_str));
        Ok(power_on == 1)
    }
}
