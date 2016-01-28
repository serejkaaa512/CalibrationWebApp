#[macro_use] 
extern crate nickel;
extern crate calibration_web_app;
extern crate url;
extern crate groupable;
extern crate mysql;

use calibration_web_app::{Generator, PowerMeter, MysqlMiddleware, MysqlRequestExtensions};
use std::collections::HashMap;
use std::io::Read;
use nickel::{Nickel, HttpRouter, Request, Response, StaticFilesHandler, MiddlewareResult};
use nickel::extensions::Redirect;
use url::form_urlencoded;
use groupable::Groupable;
use mysql::value::from_row;

struct Props {
    ip: String,
    port: u16,
    busy: bool,
    id: i32
}

fn logger<'a, D>(request: &mut Request<D>, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
    println!("logging request: {:?}", request.origin.uri);
    response.next_middleware()
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(logger);

    server.utilize(StaticFilesHandler::new("assets/"));

    server.utilize(MysqlMiddleware::new("calibr", "root", "1234"));

    server.utilize(router!(
        get "/" => |request, response| {
            let pool = request.db_connection();
            let generators: Vec<Props> = 
            pool.prep_exec("SELECT IP, Port, IsBusy, Id FROM generators", ())
            .map(|result| { result.map(|x| x.unwrap()).map(|row| {
                let (ip, port, busy, id) = from_row(row);
                Props {
                    ip: ip,
                    port: port,
                    busy: busy,
                    id:id
                }})
            .collect()})
            .unwrap(); 

            let gen_data: Vec<_> =  generators.into_iter()
            .map(|gen| {
                let mut g = HashMap::new();
                g.insert("name", format!("{}:{}, busy={}", gen.ip, gen.port, gen.busy));
                g.insert("value", format!("{}",gen.id));
                g})
            .collect();

            

            let mut data = HashMap::new();
            data.insert("generators", gen_data);
            return response.render("src/templates/mainpage.tpl", &data);
        }

        post "/" => |req, resp| {
            let mut form_data = String::new();
            req.origin.read_to_string(&mut form_data).unwrap();
            let map = &get_hashmap_from_query(&*form_data);
            let _gen = Generator::new(
                &*get_param_from_hashmap(map, "generator_ip"),
                get_param_from_hashmap(map, "generator_port").parse::<u16>().unwrap());
            let _pm = PowerMeter::new(
                &*get_param_from_hashmap(map, "powermeter_ip"),
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
