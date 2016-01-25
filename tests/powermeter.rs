extern crate calibration_web_app;
use calibration_web_app::powermeter;

fn can_create_pm(address: &str, port: u16) -> powermeter::PowerMeter{
    let pw = powermeter::PowerMeter::new(address, port);
    assert!(true);
    pw
}

fn can_get_current_power_pm(pm: &mut powermeter::PowerMeter) {
    let _power = pm.get_power(1).unwrap();
    assert!(true);
}

fn can_get_current_power_pm_err(pm: &mut powermeter::PowerMeter) {
    let _power = pm.get_power(2).unwrap();
    assert!(true);
}

#[test]
#[should_panic(expected = "Data corrupt")]
fn test_powermeter() {
    let mut pm = can_create_pm("10.10.0.7", 3333);
    can_get_current_power_pm(&mut pm);
    can_get_current_power_pm_err(&mut pm);
}
