extern crate mysql;
extern crate nickel;
extern crate typemap;
extern crate plugin;

mod generator;
mod powermeter;
mod middleware;

pub use self::generator::Generator;
pub use self::powermeter::PowerMeter;
pub use self::middleware::{MysqlMiddleware, MysqlRequestExtensions};

use std::net::TcpStream;
use std::io::{ Write, BufReader, BufRead};
use std::time::Duration;
use std::error::Error;
use std::str::FromStr;
use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug)]
pub struct Unit {
    stream: TcpStream, 
}

impl Unit{
    fn send(&mut self, buf: &[u8]) -> Result<usize, Box<Error>>{
        let result: usize = try!(self.stream.write(buf));
        try!(self.stream.flush());
        Ok(result)
    }

    fn read_line(&mut self) -> Result<String,  Box<Error>>{
        let mut reader = BufReader::new(&self.stream);
        let mut buffer = String::new();
        try!(reader.read_line(&mut buffer));
        Ok(buffer)
    }

    fn query(&mut self, buf: &[u8]) -> Result<String, Box<Error>>{
        try!(self.send(buf));
        let result = try!(self.read_line());
        Ok(result)
    }

    pub fn set_query<T: FromStr + PartialEq + Display>(&mut self, value: T, set: &str, get: &str) -> Result<T, Box<Error>> 
            where <T as FromStr>::Err: 'static + Error {
        try!(self.send(b"*CLS\n"));
        try!(self.send(set.as_bytes()).map_err(|_| self.get_errors()));

        let res = try!(self.query(get.as_bytes()).map_err(|_| self.get_errors()));
        let res_v: T = try!(res.trim().parse());
        if res_v != value {
            Err(From::from(format!("Устанавливаемое значение: {}, получено в ответ: {}.", value, res_v)))
        } else {
            Ok(res_v)
        }
    }

    pub fn get_query<T: FromStr>(&mut self, get: &str) -> Result<T, Box<Error>> 
            where <T as FromStr>::Err: 'static + Error {
        try!(self.send(b"*CLS\n"));
        let res = try!(self.query(get.as_bytes()).map_err(|_| self.get_errors()));
        let res_f: T = try!(res.trim().parse());
        Ok(res_f)
    }

    pub fn get_errors(&mut self) -> String {
        self.query(b"SYST:ERR?\n").unwrap()
    }

    pub fn get_idn(&mut self) -> String {
        self.query(b"*IDN?\n").unwrap()
    }


}

pub fn connect(address: &str, port: u16) -> Result<Unit,  Box<Error>> {
    let result: TcpStream = try!(TcpStream::connect((address, port))); 
    let timeout: Option<Duration> = Some(Duration::new(5,0));
    let _ = result.set_write_timeout(timeout);
    let _ = result.set_read_timeout(timeout);
    Ok(Unit { stream: result } )
}
