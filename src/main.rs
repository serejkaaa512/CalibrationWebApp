#[macro_use] 
extern crate nickel;
extern crate calibration_web_app;
use std::collections::HashMap;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware!{ |_,response|
        let mut data = HashMap::<&str, &str>::new();
        return response.render("src/templates/mainpage.tpl", &data);
    });

    server.listen("127.0.0.1:6767");
}


#[test]
fn server_test() {
    assert!(true);
}
