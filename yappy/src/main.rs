use std::net::{SocketAddr, TcpStream, IpAddr};
use rand::seq::SliceRandom;
use dns_lookup::{lookup_host, getnameinfo};
use std::str::FromStr;
use std::thread;

extern crate clap;
use clap::{Arg, App};
extern crate regex;
use regex::Regex;
extern crate log;
use log::{debug, trace};
extern crate simple_logger;

const MAXPORT: i32 = 65535;
const SCAN_BATCH: usize = 64;

fn tcp_port_connect(host: &str, port: i32) -> bool {
    let hostport = format!("{}:{}", host, port);

    trace!("Starting connection to {}:{}", host, port);
    if let Ok(_stream) = TcpStream::connect(hostport) {
        debug!("Got open port for {}:{}", host, port);
        return true;
    } else {
        trace!("Got closed port for {}:{}", host, port);
        return false;
    }
}

/// port range can be expressed as 1-25 or 1:25
fn parse_port(port_range: &str) -> Vec<i32> {
    let mut vec = Vec::with_capacity(1);

    let re = Regex::new(r"(\d+)?[-:](\d+)?").unwrap();
    let captures = re.captures(port_range).unwrap();

    let start = captures.get(1).map_or(1, |m| m.as_str().parse::<i32>().unwrap());
    let end = captures.get(2).map_or(MAXPORT, |m| m.as_str().parse::<i32>().unwrap());

    debug!("Start port: {}", start);
    debug!("End port: {}", end);

    vec.push(start);
    vec.push(end);
    return vec;
}

fn main() {

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
    log::debug!("Port range parsed");


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
        3 | _ => println!("No... that's too many"),
    }
    debug!("Yappy loaded");

    //let ip = matches.value_of("host").unwrap();

    let ips: Vec<std::net::IpAddr> = lookup_host(matches.value_of("host").unwrap()).unwrap();

    debug!("Found {} ips for {}", ips.len(), matches.value_of("host").unwrap());

    //TODO this is redunant, just use stdlib lookups
    let ip = match () {
        _ if ips.len() < 1 => *ips.choose(&mut rand::thread_rng()).unwrap(), //.to_string().as_str(),
        _ if ips.len() == 1 => ips[0],//.to_string().as_str(),
        _   => IpAddr::from_str(matches.value_of("host").unwrap()).unwrap(),
    };

    debug!("Using IP address {}", ip.to_string().as_str());

    let protocol = "tcp";

    let mut port_succ = 0;
    let mut port_fail = 0;

    let port_range = portrange[0]..portrange[1];

    //let port_vector = port_range.collect::<Vec<i32>>();
    let port_vector: Vec<_> = port_range.collect();

    //let port_slices: Vec<&[i32]> = port_vector.chunks(64).collect();
    let port_slices = port_vector.chunks(2);

    let mut handles = vec![];

    for port_slice in port_slices {
        let port_slice_vec = port_slice.to_vec();
        let handle = thread::spawn(move || {
            for port_range_port in port_slice_vec {
                trace!("{} port {} CONNECT", protocol, port_range_port);
                if tcp_port_connect(ip.to_string().as_str(), port_range_port) {

                    // Do lookup to get service name for output
                    let socket: SocketAddr = format!("{}:{}", ip.to_string(), port_range_port)
                        .parse()
                        .expect("Unable to parse socket address");
                    let (name, service) = match getnameinfo(&socket, 0) {
                        Ok((n, s)) => (n, s),
                        Err(e) => panic!("Failed to lookup socket {:?}", e),
                    };

                    debug!("{} port {}:{} [{}] open", protocol, name, port_range_port, service);
                    port_succ += 1;
                } else {
                    port_fail += 1;
                    trace!("{} port {} closed", protocol, port_range_port);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{} ports open", port_succ);
    debug!("{} ports closed", port_fail);
}
