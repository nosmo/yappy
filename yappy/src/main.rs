use std::net::TcpStream;

extern crate clap;
use clap::{Arg, App};
extern crate regex;
use regex::Regex;
extern crate log;
use log::{debug, trace};
extern crate simple_logger;
use simple_logger::SimpleLogger;

// this is a comment
/* this is a comment */
/// this one is a docstring that is general

const MAXPORT: i32 = 65535;

fn tcp_port_connect(host: &str, port: i32) -> bool {
    let hostport = format!("{}:{}", host, port);

    if let Ok(_stream) = TcpStream::connect(hostport) {
        debug!("TCP port {} open", port);
        return true;
    } else {
        trace!("TCP port {} closed", port);
        return false;
    }
}

fn parse_port(port_range: &str) -> Vec<i32> {
    let mut vec = Vec::with_capacity(1);
    println!("lmao sup");

    let re = Regex::new(r"(\d*)-(\d*)").unwrap();
    let captures = re.captures(port_range).unwrap();
    //FIXME this doesn't work as expected - null parameters cause an
    // error because we can't parse a int on an empty value.
    let start = captures.get(1).map_or(1, |m| m.as_str().parse::<i32>().unwrap());
    println!("Start port: {}", start);
    let end = captures.get(2).map_or(MAXPORT, |m| m.as_str().parse::<i32>().unwrap());
    println!("End port: {}", end);

    vec.push(start);
    vec.push(end);
    return vec;
    //vec.push(;
}

fn main() {

    //! this is a docstring that is line specific
    let matches = App::new("yappy")
        .version("1.0")
        .author("hugh <nosmo@nosmo.me>")
        .about("Yet Another Portscanner Programming Yoject")
        .arg(Arg::with_name("host")
             .help("The host to scan")
             .required(true)
             .index(1))
        .arg(Arg::with_name("portrange")
             .short("p")
             .help("Port range to scan. Format: START-END, -END, START-")
             .default_value("-")
             .takes_value(true)
             .hide_default_value(true))
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Print verbose output"))
        .get_matches();

    let portrange = parse_port(matches.value_of("portrange").unwrap());

    match matches.occurrences_of("v") {
        0 => simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Error)
            .init()
            .unwrap(),
        1 => simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .init()
            .unwrap(),
        2 => simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap(),
        3 | _ => println!("Don't be crazy"),
    }

    let ip = matches.value_of("host").unwrap();

    let protocol = "tcp";

    let mut port_succ = 0;
    let mut port_fail = 0;

    let port_range = portrange[0]..portrange[1];

    for port_range_port in port_range {
        if tcp_port_connect(ip, port_range_port) {
            port_succ += 1;
        } else {
            port_fail += 1;
        }
    }

    println!("{} ports open", port_succ);
    println!("{} ports closed", port_fail);
}
