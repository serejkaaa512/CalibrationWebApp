#[macro_use] 
extern crate nickel;
extern crate calibration_web_app;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Calibration start!"
        }
    });

    server.listen("127.0.0.1:6767");
}
