#[macro_use] 
extern crate nickel;
extern crate calibration_web_app;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware!("<html><title>Calibration!!!</title></html>"));

    server.listen("127.0.0.1:6767");
}


#[test]
fn server_test() {
    assert!(true);
}
