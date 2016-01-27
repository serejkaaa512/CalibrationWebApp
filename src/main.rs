#[macro_use] 
extern crate nickel;
extern crate calibration_web_app;
extern crate url;
extern crate groupable;

use calibration_web_app::generator::Generator;
use calibration_web_app::powermeter::PowerMeter;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, Request, Response, StaticFilesHandler,
    MiddlewareResult};
    use nickel::extensions::Redirect;
    use std::io::Read;
    use url::form_urlencoded;
    use groupable::Groupable;

    struct Devices {
        gen: Generator,
        pm: PowerMeter
    }

    impl Devices {
        pub fn new(gen_ip: String, gen_port: u16, pm_ip: String, pm_port: u16) -> Devices {
            Devices { 
                gen: Generator::new(&*gen_ip, gen_port),
                pm: PowerMeter::new(&*pm_ip, pm_port), 
            }
        }
    }


    fn logger<'a, D>(request: &mut Request<D>, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
        println!("logging request: {:?}", request.origin.uri);
        response.next_middleware()
    }

    fn main() {
        let mut server = Nickel::new();

        server.utilize(logger);

        server.utilize(StaticFilesHandler::new("assets/"));

        server.utilize(router!(
            get "/" => |_,response| {
                let mut _data = HashMap::<&str, &str>::new();
                return response.render("src/templates/mainpage.tpl", &_data);
            }

            post "/" => |req, resp| {
                let mut form_data = String::new();
                req.origin.read_to_string(&mut form_data).unwrap();
                let map = &get_hashmap_from_query(&*form_data);
                Devices::new(
                    get_param_from_hashmap(map, "generator_ip"),
                    get_param_from_hashmap(map, "generator_port").parse::<u16>().unwrap(),
                    get_param_from_hashmap(map, "powermeter_ip"),
                    get_param_from_hashmap(map, "powermeter_port").parse::<u16>().unwrap());
                return resp.redirect("/calibration/")
            }

        ));

        server.listen("0.0.0.0:6767");

    }


    fn get_hashmap_from_query(encoded_string : &str) -> HashMap<String, Vec<String>>{
        form_urlencoded::parse(encoded_string.as_bytes())
        .into_iter()
        .group()
    }

    fn get_param_from_hashmap(map: &HashMap<String, Vec<String>>, param_name: &str) -> String{
        map.get(param_name)
        .and_then(|v| v.first().map(|s| &**s))
        .unwrap()
        .to_string()
    }
