extern crate calibration_web_app;
use calibration_web_app::generator;

fn can_create_gen(address: &str, port: u16) -> generator::Generator{
    let pw = generator::Generator::new(address, port);
    assert!(true);
    pw
}

fn can_get_current_freq(gen: &mut generator::Generator) {
    let _freq = gen.get_freq().unwrap();
    assert!(true);
}

fn can_get_current_power(gen: &mut generator::Generator) {
    let _power = gen.get_power().unwrap();
    assert!(true);
}

fn can_set_current_freq(gen: &mut generator::Generator, freq: f32) {
    let _freq = gen.set_freq(freq).unwrap();
    assert!(true);
}

fn can_set_current_power(gen: &mut generator::Generator, power: f32) {
    let _power = gen.set_power(power).unwrap();
    assert!(true);
}

fn can_set_current_power_on(gen: &mut generator::Generator) {
    let _power = gen.set_power_on().is_none();
    assert!(_power);
}

fn can_set_current_power_off(gen: &mut generator::Generator) {
    let _power = gen.set_power_off().is_none();
    assert!(_power);
}

fn can_get_current_power_on(gen: &mut generator::Generator) {
    let _power = gen.get_power_on().unwrap();
    assert!(true);
}

#[test]
fn test_generator() {
    let mut gen = can_create_gen("10.10.0.7", 4444);
    can_get_current_freq(&mut gen);
    can_set_current_freq(&mut gen, 123456.0);
    can_get_current_power(&mut gen);
    can_set_current_power(&mut gen, 0f32);
    can_set_current_power_on(&mut gen);
    can_get_current_power_on(&mut gen);
    can_set_current_power_off(&mut gen);
}
