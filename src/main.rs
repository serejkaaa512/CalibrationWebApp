#[macro_use]
extern crate nickel;
extern crate calibration_web_app;
extern crate url;
extern crate groupable;
extern crate mysql;
extern crate rustc_serialize;
extern crate time;
extern crate regex;
extern crate simple_chart;

use regex::Regex;
use calibration_web_app::{Generator, PowerMeter, MysqlMiddleware, MysqlRequestExtensions};
use mysql::conn::pool::Pool;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use nickel::{Nickel, Request, Response, StaticFilesHandler, MiddlewareResult, QueryString,
             JsonBody};
use nickel::status::StatusCode;
use url::form_urlencoded;
use mysql::value::from_row;
use simple_chart::{Chart, Serie, Point};
#[derive(RustcDecodable, RustcEncodable, Clone)]
struct Props {
    ip: String,
    port: u16,
    busy: bool,
    id: u64,
}

#[derive(RustcDecodable, RustcEncodable)]
struct Report {
    id: i32,
    name: String,
    values: Vec<FreqPower>,
}

#[derive(RustcDecodable, RustcEncodable, Clone, PartialEq)]
struct FreqPower {
    freq: f32,
    pow: f32,
}

impl<'a> From<FreqPower> for Point {
    fn from(t: FreqPower) -> Point {
        Point {
            x: t.freq as f64,
            y: t.pow as f64,
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

    server.utilize(MysqlMiddleware::new("calibr", "root", "1234"));

    let set_freq_regex = Regex::new(r"/generator/(?P<id>\d+)/set_freq/(?P<freq>[-+]?(\d*[.])?\d+)")
        .unwrap();

    let set_pow_regex = Regex::new(r"/generator/(?P<id>\d+)/set_power/(?P<p>[-+]?(\d*[.])?\d+)")
        .unwrap();

    server.utilize(router!(

        get "/" => |req, resp| {
            return get_all_devices(req, resp)
        }

        get "/calibration/options" => |req, resp| {
            return get_report_options(req, resp)
        }

        get "/calibration/algorithm" => |req, resp| {
            return calibration_algorithm(req, resp)
        }
        
        get "/calibration/reports" => |req, resp| {
            return get_reports(req, resp)
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

        post set_pow_regex => |req, resp| {
            return set_generator_power(req, resp)
        }

        post "/generator/:id/turn_on" => |req, resp| {
            return turn_generator_on(req, resp)
        }

        post "/generator/:id/turn_off" => |req, resp| {
            return turn_generator_off(req, resp)
        }
        
        post set_freq_regex => |req, resp| {
            return set_generator_freq(req, resp)
        }

        get "/powermeter/:id/:channel/power" => |req, resp| {
            return get_powermeter_power(req, resp)
        }

        post "/report/add" => |req, resp| {
            return add_report(req, resp)
        }
        
        get "/calibration/report" => |req, resp| {
            return get_report(req, resp)
        }

        get "/calibration/graph/:id" => |req, resp| {
            return get_graph(req, resp)
        }

    ));

    server.listen("0.0.0.0:6767").unwrap();
}


fn get_graph<'a>(req: &mut Request, resp: Response<'a>) -> MiddlewareResult<'a> {
    let pool = req.db_connection();
    let id = req.param("id").unwrap();
    
    let select_str = format!("SELECT * FROM report_{}", id);
    let res: Vec<FreqPower> = pool.prep_exec(select_str, ())
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let values: (f32, f32) = from_row(row);
                    FreqPower {
                        freq: values.0,
                        pow: values.1,
                    }
                })
                .collect()
        })
        .unwrap();

    let mut chart = Chart::new(740, 480, "#ffffff", "#000000")
        .unwrap();
    match Serie::new(res.into_iter(), "#ff0000") {
        Ok(serie) => {
            let series = vec![serie];
            let bmp = chart.draw(series.into_iter());
            resp.send(bmp)
        },
        Err(err) => resp.send(err.to_string())
    }
}


fn get_report<'a>(req: &mut Request, resp: Response<'a>) -> MiddlewareResult<'a> {
    let pool = req.db_connection();
    let query = req.query();
    let table_name = query.get("name").unwrap();
    let select_table_id = format!("SELECT id FROM reports WHERE name='{}'", table_name);
    let id: Vec<i32> = pool.prep_exec(select_table_id, ())
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(from_row)
                .collect()
        })
        .unwrap();

    let select_str = format!("SELECT * FROM report_{}", id[0].clone());
    let res: Vec<FreqPower> = pool.prep_exec(select_str, ())
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let values: (f32, f32) = from_row(row);
                    FreqPower {
                        freq: values.0,
                        pow: values.1,
                    }
                })
                .collect()
        })
        .unwrap();
    let mut data = HashMap::new();
    let report = Report {
        id: id[0].clone(),
        name: table_name.to_string(),
        values: res,
    };
    data.insert("report", report);

    resp.render("templates/report.tpl", &data)
}



fn get_reports<'a>(req: &mut Request, resp: Response<'a>) -> MiddlewareResult<'a> {
    let pool = req.db_connection();
    let select_str = "SELECT Name FROM reports LIMIT 10";
    let res: Vec<Report> = pool.prep_exec(select_str, ())
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let name: String = from_row(row);
                    Report {
                        id: 0,
                        name: name,
                        values: (Vec::new()),
                    }
                })
                .collect()
        })
        .unwrap();
    let mut data: HashMap<&str, Vec<Report>> = HashMap::new();
    data.insert("reps", res);
    resp.render("templates/reports.tpl", &data)
}


fn add_report<'a>(req: &mut Request, resp: Response<'a>) -> MiddlewareResult<'a> {
    let pool = req.db_connection();
    let report = req.json_as::<Report>().unwrap();
    for v in report.values {
        let insert_str = format!("INSERT INTO `{}`(F, P) VALUES (?, ?)", report.name);
        let mut stmt = pool.prepare(insert_str).unwrap();
        let _ = stmt.execute((v.freq, v.pow));
    }
    resp.send("Сохранено в бд!")
}


fn get_powermeter(id: &str, pool: &Arc<Pool>) -> Option<PowerMeter> {
    let pm_props = get_device_from_db(pool, "powermeters", id);
    PowerMeter::new(&*(pm_props.ip), pm_props.port)
}

fn get_generator(id: &str, pool: &Arc<Pool>) -> Option<Generator> {
    let gen_props = get_device_from_db(pool, "generators", id);
    Generator::new(&*(gen_props.ip), gen_props.port)
}

fn get_powermeter_power<'a>(req: &mut Request, mut resp: Response<'a>) -> MiddlewareResult<'a> {
    let id = req.param("id").unwrap();
    let channel = req.param("channel").unwrap().parse::<u8>().unwrap();
    let pool = req.db_connection();
    let mut powermeter = match get_powermeter(id, &pool) {
        Some(pm) => pm,
        None => {
            resp.set(StatusCode::BadRequest);
            return resp.send("Невозможно соединиться с измерителем мощности.");
        }
    };
    let power = powermeter.get_power(channel);
    match power {
        Ok(p) => resp.send(p.to_string()),
        Err(e) => {
            resp.set(StatusCode::BadRequest);
            resp.send(e.to_string())
        }
    }
}

fn set_generator_freq<'a>(req: &mut Request, mut resp: Response<'a>) -> MiddlewareResult<'a> {
    let id = req.param("id").unwrap();
    let freq = req.param("freq").unwrap().parse::<f32>().unwrap();
    let pool = req.db_connection();
    let mut generator = match get_generator(id, &pool) {
        Some(gen) => gen,
        None => {
            resp.set(StatusCode::BadRequest);
            return resp.send("Невозможно соединиться с генератором.");
        }
    };
    match generator.set_freq(freq) {
        Ok(_) => resp.send("freq is setted!"),
        Err(e) => {
            resp.set(StatusCode::BadRequest);
            resp.send(e.to_string())
        }
    }
}

fn turn_generator_off<'a>(req: &mut Request, mut resp: Response<'a>) -> MiddlewareResult<'a> {
    let id = req.param("id").unwrap();
    let pool = req.db_connection();
    let mut generator = match get_generator(id, &pool) {
        Some(gen) => gen,
        None => {
            resp.set(StatusCode::BadRequest);
            return resp.send("Невозможно соединиться с генератором.");
        }
    };
    match generator.set_power_off() {
        Some(e) => {
            resp.set(StatusCode::BadRequest);
            resp.send(e.to_string())
        }
        None => resp.send("power is turned off!"),
    }
}


fn turn_generator_on<'a>(req: &mut Request, mut resp: Response<'a>) -> MiddlewareResult<'a> {
    let id = req.param("id").unwrap();
    let pool = req.db_connection();
    let mut generator = match get_generator(id, &pool) {
        Some(gen) => gen,
        None => {
            resp.set(StatusCode::BadRequest);
            return resp.send("Невозможно соединиться с генератором.");
        }
    };
    match generator.set_power_on() {
        Some(e) => {
            resp.set(StatusCode::BadRequest);
            resp.send(e.to_string())
        }
        None => resp.send("power is turned on!"),
    }
}

fn set_generator_power<'a>(req: &mut Request, mut resp: Response<'a>) -> MiddlewareResult<'a> {
    let id = req.param("id").unwrap();
    let p = req.param("p").unwrap().parse::<f32>().unwrap();
    let pool = req.db_connection();
    let mut generator = match get_generator(id, &pool) {
        Some(gen) => gen,
        None => {
            resp.set(StatusCode::BadRequest);
            return resp.send("Невозможно соединиться с генератором.");
        }
    };
    match generator.set_power(p) {
        Ok(_) => resp.send("power is setted!"),
        Err(e) => {
            resp.set(StatusCode::BadRequest);
            resp.send(e.to_string())
        }
    }
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
    let pchannel = query.get("pchannel").unwrap();

    let time = time::strftime("%d/%m/%y %H:%M:%S", &time::now()).unwrap();
    let name = &*(query.get("name").unwrap().to_string() + " " + &*time);

    let insert_str = r"INSERT INTO reports(id_gen, id_pm, Name, Fmin, Fmax, Fstep, Pgen, Pchannel) 
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
    let mut stmt = pool.prepare(insert_str).unwrap();
    let query_res = stmt.execute((gen_id, pm_id, name, fmin, fmax, fstep, pgen, pchannel));
    let id = query_res.unwrap().last_insert_id();

    let table_name = &*format!("report_{:?}", id);
    let create_str = format!("CREATE TABLE `{}` ( `F` FLOAT, `P` FLOAT)ENGINE=MyISAM",
                             table_name);
    let _ = pool.prep_exec(create_str, ());

    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("gen_id", gen_id);
    data.insert("pm_id", pm_id);
    data.insert("name", name);
    data.insert("fmin", fmin);
    data.insert("fmax", fmax);
    data.insert("fstep", fstep);
    data.insert("pgen", pgen);
    data.insert("table_name", table_name);
    data.insert("pchannel", pchannel);
    resp.render("templates/algorithm.tpl", &data)
}

fn get_report_options<'a>(req: &mut Request, resp: Response<'a>) -> MiddlewareResult<'a> {
    let pool = req.db_connection();
    let query = req.query();
    let gen_id = query.get("generator_id").unwrap();
    let pm_id = query.get("powermeter_id").unwrap();
    let generator = get_device_from_db(&pool, "generators", gen_id);
    let powermeter = get_device_from_db(&pool, "powermeters", pm_id);
    let mut data: HashMap<&str, Props> = HashMap::new();
    data.insert("generator", generator);
    data.insert("powermeter", powermeter);
    resp.render("templates/options.tpl", &data)
}

fn get_device_from_db(pool: &Arc<Pool>, dev_name: &str, id: &str) -> Props {
    let res: Vec<Props> =
        pool.prep_exec(format!("SELECT IP, Port, IsBusy, Id FROM {} WHERE id=?", dev_name),
                       (id,))
            .map(|result| {
                result.map(|x| x.unwrap())
                    .map(|row| {
                        let (ip, port, busy, id) = from_row(row);
                        Props {
                            ip: ip,
                            port: port,
                            busy: busy,
                            id: id,
                        }
                    })
                    .collect()
            })
            .unwrap();
    res[0].clone()
}


fn get_all_devices<'a>(req: &mut Request, resp: Response<'a>) -> MiddlewareResult<'a> {
    let pool = req.db_connection();
    let generators = get_devices_from_db(&pool, "generators");
    let powermeters = get_devices_from_db(&pool, "powermeters");
    let mut data: HashMap<&str, Vec<Props>> = HashMap::new();
    data.insert("generators", generators);
    data.insert("powermeters", powermeters);
    resp.render("templates/mainpage.tpl", &data)
}

fn get_devices_from_db(pool: &Arc<Pool>, dev_name: &str) -> Vec<Props> {
    pool.prep_exec(format!("SELECT IP, Port, IsBusy, Id FROM {}", dev_name), ())
        .map(|result| {
            result.map(|x| x.unwrap())
                .map(|row| {
                    let (ip, port, busy, id) = from_row(row);
                    Props {
                        ip: ip,
                        port: port,
                        busy: busy,
                        id: id,
                    }
                })
                .collect()
        })
        .unwrap()
}


fn rem_device_from_db<'a>(req: &mut Request,
                          resp: Response<'a>,
                          dev_name: &str)
                          -> MiddlewareResult<'a> {
    let id = req.param("id");
    let pool = req.db_connection();
    let _ = pool.prep_exec(format!("DELETE FROM {}s WHERE id=?", dev_name), (id,));
    resp.send("removed")
}

fn add_device_to_db<'a>(req: &mut Request,
                        resp: Response<'a>,
                        dev_name: &str)
                        -> MiddlewareResult<'a> {
    let mut form_data = String::new();
    req.origin.read_to_string(&mut form_data).unwrap();
    let mut data = HashMap::new();
    for (key, value) in form_urlencoded::parse(form_data.as_bytes()) {
        data.insert(key, value);
    }
    let ip = &*data.get("ip").unwrap().parse::<String>().unwrap();
    let port = data.get("port").unwrap().parse::<u16>().unwrap();
    let pool = req.db_connection();
    let insert_str = format!(r"INSERT INTO {}s(IP, Port, IsBusy) VALUES (?, ?, ?)",
                             dev_name);
    let mut stmt = pool.prepare(insert_str).unwrap();
    let query = stmt.execute((ip, port, false));
    let id = query.unwrap().last_insert_id();
    resp.render(format!("templates/{}.tpl", dev_name),
                &Props {
                    ip: ip.to_string(),
                    port: port,
                    busy: false,
                    id: id,
                })
}
