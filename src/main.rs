#[macro_use] 
extern crate nickel;
extern crate calibration_web_app;
extern crate url;
extern crate groupable;
extern crate mysql;
extern crate rustc_serialize;
extern crate time;

use calibration_web_app::{Generator, PowerMeter, MysqlMiddleware, MysqlRequestExtensions};
use mysql::conn::pool::MyPool;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use nickel::{Nickel, HttpRouter, Request, Response, StaticFilesHandler, MiddlewareResult, QueryString};
use url::form_urlencoded;
use groupable::Groupable;
use mysql::value::from_row;

#[derive(RustcDecodable, RustcEncodable, Clone)]
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

#[derive(RustcDecodable, RustcEncodable)]
struct Options {
    generator: Props,
    powermeter: Props,
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
        get "/" => |req, resp| {
            return get_all_devices(req, resp)
        }

        get "/calibration/options" => |req, resp| {
            return get_report_options(req, resp)
        }

        get "/calibration/algorithm/" => |req, resp| {
            return calibration_algorithm(req, resp)
        }
        
        post "/generator/add" => |req, resp| {
            return add_device_to_db(req, resp, "generator")
        }

        post "/powermeter/add" => |req, resp| {
            return add_device_to_db(req, resp, "powermeter")
        }

        delete "/generator/:id" => |req, resp| {
            return rem_device_from_db(req, resp, "generator")
        }

        delete "/powermeter/:id" => |req, resp| {
            return rem_device_from_db(req, resp, "powermeter")
        }
        ));
    server.listen("0.0.0.0:6767");
}


fn calibration_algorithm<'a>(req: &mut Request, resp: Response<'a>) -> MiddlewareResult<'a> {
    let pool = req.db_connection();
    let query = req.query();
    let gen_id = query.get("gen_id").unwrap();
    let pm_id = query.get("pm_id").unwrap();
    let fmin = query.get("fmin").unwrap();
    let fmax = query.get("fmax").unwrap();
    let fstep = query.get("fstep").unwrap();
    let pgen = query.get("pgen").unwrap();
    
    let name = query.get("name").unwrap();
    let time = time::strftime("%y_%m_%d__%H_%M_%S", &time::now()).unwrap();
    let table_name = &*(name.to_string() + "_" + &*time);

    let insert_str = 
    r"INSERT INTO reports(id_gen, id_pm, Name, Fmin, Fmax, Fstep, Pgen) VALUES (?, ?, ?, ?, ?, ?, ?)";
    let mut stmt = pool.prepare(insert_str).unwrap();
    let _ = stmt.execute((gen_id, pm_id, table_name, fmin, fmax, fstep, pgen));

    let create_str = format!(
    "CREATE TABLE `{}` ( `F` FLOAT, `P` FLOAT)ENGINE=MyISAM", table_name);
    let _ = pool.prep_exec(create_str, ());

    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("name", name);
    data.insert("fmin", fmin);
    data.insert("fmax", fmax);
    data.insert("fstep", fstep);
    data.insert("pgen", pgen);
    data.insert("table_name", table_name);
    return resp.render("src/templates/algorithm.tpl", &data);
}

fn get_report_options<'a>(req: &mut Request, resp: Response<'a>) -> MiddlewareResult<'a> {
    let pool = req.db_connection();
    let query = req.query();
    let gen_id = query.get("generator_id").unwrap();
    let pm_id = query.get("powermeter_id").unwrap();
    let generator = get_device_from_db(&pool, "generators", gen_id);
    let powermeter = get_device_from_db(&pool, "powermeters", pm_id);
    let data = Options {generator: generator, powermeter: powermeter};
    return resp.render("src/templates/options.tpl", &data)
}

fn get_device_from_db(pool: &Arc<MyPool>, dev_name: &str, id: &str) -> Props {
    let res: Vec<Props> = 
    pool.prep_exec(format!("SELECT IP, Port, IsBusy, Id FROM {} WHERE id=?", dev_name), (id,))
    .map(|result| { result.map(|x| x.unwrap()).map(|row| {
        let (ip, port, busy, id) = from_row(row);
        Props {ip: ip, port: port, busy: busy, id: id}}).collect()}).unwrap();
    res[0].clone()
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


fn get_all_devices<'a>(req: &mut Request, resp: Response<'a>) -> MiddlewareResult<'a> {
    let pool = req.db_connection();
    let generators = get_devices_from_db(&pool, "generators");
    let powermeters = get_devices_from_db(&pool, "powermeters");
    let data = Devices{ generators: generators, powermeters: powermeters };
    return resp.render("src/templates/mainpage.tpl", &data);
}

fn get_devices_from_db(pool: &Arc<MyPool>, dev_name: &str) -> Vec<Props> {
    pool.prep_exec(format!("SELECT IP, Port, IsBusy, Id FROM {}", dev_name), ())
    .map(|result| { result.map(|x| x.unwrap()).map(|row| {
        let (ip, port, busy, id) = from_row(row);
        Props {ip: ip, port: port, busy: busy, id: id}}).collect()}).unwrap()
}


fn rem_device_from_db<'a>(req: &mut Request, resp: Response<'a>, dev_name: &str) -> MiddlewareResult<'a> {
    let id = req.param("id");
    let pool = req.db_connection();
    let _ = pool.prep_exec(format!("DELETE FROM {}s WHERE id=?", dev_name),(id,));
    resp.send("removed")
}

fn add_device_to_db<'a>(req: &mut Request, resp: Response<'a>, dev_name: &str) -> MiddlewareResult<'a> {
    let mut form_data = String::new();
    req.origin.read_to_string(&mut form_data).unwrap();
    let map = &get_hashmap_from_query(&*form_data);
    let ip = &*get_param_from_hashmap(map, "ip");
    let port = get_param_from_hashmap(map, "port").parse::<u16>().unwrap();
    let pool = req.db_connection();
    let insert_str = 
    format!(r"INSERT INTO {}s(IP, Port, IsBusy) VALUES (?, ?, ?)", dev_name);
    let mut stmt = pool.prepare(insert_str).unwrap();
    let _ = stmt.execute((ip,port,false));

    let select_str = 
    format!("SELECT IP, Port, IsBusy, Id FROM {}s WHERE IP=? AND Port=? AND IsBusy=false",
        dev_name);
    let res: Vec<Props> = pool.prep_exec(select_str, (ip, port))
    .map(|result| { result.map(|x| x.unwrap()).map(|row| {
        let (ip, port, busy, id) = from_row(row);
        Props {ip: ip, port: port, busy: busy, id: id}}).collect()})
    .unwrap();

    return resp.render(format!("src/templates/{}.tpl", dev_name), &res[0]);
}
