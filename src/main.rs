#[macro_use] 
extern crate nickel;
extern crate calibration_web_app;
extern crate url;
extern crate groupable;
extern crate mysql;
extern crate rustc_serialize;

use calibration_web_app::{Generator, PowerMeter, MysqlMiddleware, MysqlRequestExtensions};
use mysql::conn::pool::MyPool;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use nickel::{Nickel, HttpRouter, Request, Response, StaticFilesHandler, MiddlewareResult};
use nickel::extensions::Redirect;
use url::form_urlencoded;
use groupable::Groupable;
use mysql::value::from_row;

#[derive(RustcDecodable, RustcEncodable)]
struct Props {
    ip: String,
    port: u16,
    busy: bool,
    id: i32
}

#[derive(RustcDecodable, RustcEncodable)]
struct Devices {
    generators: Vec<Props>,
    powermeters: Vec<Props>,
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
            let generators = get_devices_from_db(&pool, "generators");
            let powermeters = get_devices_from_db(&pool, "powermeters");
            let data = Devices{ generators: generators, powermeters: powermeters };
            return response.render("src/templates/mainpage.tpl", &data);
        }

        post "/" => |req, resp| {
            let mut form_data = String::new();
            req.origin.read_to_string(&mut form_data).unwrap();
            let map = &get_hashmap_from_query(&*form_data);

            if map.contains_key("add_generator") {
                let pool = req.db_connection();
                add_device_to_db(map, pool, "generator");
                return resp.redirect("/")
            }

            if let Some(k) = map.keys().find(|ref key| key.contains("rem_generator")){
                let pool = req.db_connection();
                rem_device_from_db(pool, "generator", k);
                return resp.redirect("/")
            }
            
            if map.contains_key("add_powermeter") {
                let pool = req.db_connection();
                add_device_to_db(map, pool, "powermeter");
                return resp.redirect("/")
            }

            if let Some(k) = map.keys().find(|ref key| key.contains("rem_powermeter")){
                let pool = req.db_connection();
                rem_device_from_db(pool, "powermeter", k);
                return resp.redirect("/")
            }

            println!("{:?}", map);

            return resp.redirect("/calibration/")
        }
        ));server.listen("0.0.0.0:6767");
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

fn add_device_to_db(map: &HashMap<String, Vec<String>>, pool: Arc<MyPool>, dev_name: &str) {
    let mut stmt = 
    pool.prepare(format!(r"INSERT INTO {}s(IP, Port, IsBusy) VALUES (?, ?, ?)", dev_name)).unwrap();
    stmt.execute((
        &*get_param_from_hashmap(map, &*format!("{}_ip",dev_name)), 
        get_param_from_hashmap(map, &*format!("{}_port",dev_name)).parse::<u16>().unwrap(), 
        false)).unwrap();
}

fn rem_device_from_db(pool: Arc<MyPool>, dev_name: &str, k: &str) {
    let mut id = (*k).to_string();
    let offset = id.find('-').unwrap() + 1;
    id.drain(..offset).collect::<String>();
    let _ = pool.prep_exec(format!("DELETE FROM {}s WHERE id=?", dev_name),(id,));
}

fn get_devices_from_db(pool: &Arc<MyPool>, dev_name: &str) -> Vec<Props> {
    pool.prep_exec(format!("SELECT IP, Port, IsBusy, Id FROM {}", dev_name), ())
    .map(|result| { result.map(|x| x.unwrap()).map(|row| {
        let (ip, port, busy, id) = from_row(row);
        Props {ip: ip, port: port, busy: busy, id: id}}).collect()}).unwrap()
}
