use super::Unit;
use std::error::Error;

#[derive(Debug)]
pub struct Generator {
    unit: Unit
}

impl Generator {
    pub fn new(ip: & str, port: u16) -> Option<Generator> {
        let unit = super::connect(ip, port);
        match unit {
            Ok(unit) => Some(Generator { unit: unit }),
            Err(_) => None,
        }
    }

    pub fn get_freq(&mut self) -> Result<f32, Box<Error>> {
        self.unit
        .get_query("SOUR:FREQ:FIX?\n")
        .map_err(|e| From::from(
            format!("Не удалось получить значение текущей частоты с генератора.<br>{}", e.to_string())
            )
        )
        .and_then(|f:f32| Ok(f/1000000.0))
    }

    pub fn get_power(&mut self) -> Result<f32, Box<Error>> {
        self.unit.get_query("SOUR:POW:LEV:IMM:AMPL?\n")
        .map_err(|e| From::from(
            format!("Не удалось получить значение текущей мощности с генератора.<br>{}", e.to_string())
            )
        )
    }

    pub fn set_freq(&mut self, mut freq: f32) -> Result<f32, Box<Error>> {
        freq = (freq * 1000000f32).round();
        let set_freq_str = &*format!("SOUR:FREQ:FIX {}\n", freq.to_string());
        let get_freq_str = &*format!("SOUR:FREQ:FIX?\n");
        self.unit.set_query(freq, set_freq_str, get_freq_str)
        .map_err(|e| From::from(
            format!("Не удалось установить значение текущей частоты на генераторе.<br>{}", e.to_string())
            )
        )
    }

    pub fn set_power(&mut self, power: f32) -> Result<f32, Box<Error>> {
        let set_power_str = &*format!("SOUR:POW:LEV:IMM:AMPL {}\n", power.to_string());
        let get_power_str = &*format!("SOUR:POW:LEV:IMM:AMPL?\n");
        self.unit.set_query(power, set_power_str, get_power_str)
        .map_err(|e| From::from(
            format!("Не удалось установить значение текущей мощности на генераторе.<br>{}", e.to_string())
            )
        )
    }

    pub fn set_power_on(&mut self) -> Option<Box<Error>> {
        let power = 1u16;
        let set_power_str = &*format!("OUTP:STAT {}\n", power);
        let get_power_str = &*format!("OUTP:STAT?\n");
        self.unit.set_query(power, set_power_str, get_power_str)
        .map_err(|e| From::from(
            format!("Не удалось включить мощность на генераторе.<br>{}", e.to_string())
            )
        )
        .err()
    }

    pub fn set_power_off(&mut self) -> Option<Box<Error>> {
        let power = 0u16;
        let set_power_str = &*format!("OUTP:STAT {}\n", power);
        let get_power_str = &*format!("OUTP:STAT?\n");
        self.unit.set_query(power, set_power_str, get_power_str)
        .map_err(|e| From::from(
            format!("Не удалось выключить мощность на генераторе.<br>{}", e.to_string())
            )
        )
        .err()
    }

    pub fn get_power_on(&mut self) -> Result<bool, Box<Error>> {
        let get_power_on_str = &*format!("OUTP:STAT?\n");
        let power_on: u16 = try!(self.unit.get_query(get_power_on_str));
        Ok(power_on == 1)
    }
}
