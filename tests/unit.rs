extern crate calibration_web_app;

fn unit_can_connect(address: &str, port: u16) -> calibration_web_app::Unit {
    calibration_web_app::connect(address, port).unwrap()
}

fn get_errors(unit: &mut calibration_web_app::Unit) -> String{
    unit.get_errors()
}

#[test] #[ignore]
fn unit_test() {
    let mut unit = unit_can_connect("10.10.0.193", 5025);
    get_errors(&mut unit);
}
