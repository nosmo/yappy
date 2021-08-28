use std::io::{stdout, BufWriter};
use std::env;
use std::net::TcpStream;

extern crate log;
use log::{LevelFilter, debug, trace};

extern crate clap;
use clap::{Arg, App, SubCommand};
extern crate simple_logger;
use simple_logger::SimpleLogger;

// this is a comment
/* this is a comment */
/// this one is a docstring that is general

const MAXPORT: i32 = 65535;

fn tcp_check_port(host: &str, port: i32) -> bool {
    let hostport = format!("{}:{}", host, port);

    if let Ok(_stream) = TcpStream::connect(hostport) {
        debug!("TCP port {} open", port);
        return true;
    } else {
        trace!("TCP port {} closed", port);
        return false;
    }
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
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Print verbose output"))
        .get_matches();

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

    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let ip = matches.value_of("host").unwrap();
    //let filename = &args[2];

    let port = 22;

    let port_start = 1;
    let port_end = 2000;
    //let port_end = MAXPORT;

    let protocol = "tcp";

    let mut port_succ = 0;
    let mut port_fail = 0;

    let port_range = port_start..port_end;

    for port_range_port in port_range {
        if tcp_check_port(ip, port_range_port) {
            port_succ += 1;
        } else {
            port_fail += 1;
        }
    }

    println!("{} ports open", port_succ);
    println!("{} ports closed", port_fail);
}
