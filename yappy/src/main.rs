use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

// this is a comment
/* this is a comment */
/// this one is a docstring that is general

use std::net::TcpStream;

// Chemical spill Mutation world domination Methamphetamine Malware
// industrial espionage Quarnantine Rubin Vaccine Al Qaida China Recce
// FMS Iraq Iran

// Power lines AGT. AMME Albright Jet State of emergency remailers
// Halibut Emergency management Ron Brown NORAD Vince Foster Consul
// Cartel de Golfo Weapons grade Vickie Weaver

// AIEWS ISA FCIC FX LLC Gorelick Sleet Cartel National Guard CBNRC CBP
// Drug war Fort Hancock BOP Adriatic

const MAXPORT: i32 = 65535;

fn main() {

    //! this is a docstring that is line specific
    let pi = 3.141592;
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    println!("pi is exactly {1:.0$}", 4, pi);

    //let ip = "127.0.0.1";
    let ip = "192.168.0.34";
    let port = 22;

    let port_start = 1;
    let port_end = 2000;
    //let port_end = MAXPORT;

    let protocol = "tcp";

    #[derive(Debug)]
    struct DebugPrintable(i32);

    println!("Now {:?} will print!", DebugPrintable(123));
    println!("Now {:#?} will pretty print!", DebugPrintable(123));

    println!("ACAB {} {} {} {0} {2} {1}", 1312i64,
             "butts", 123.1234);
    // the 1312i64 means that 1312 is 64 bit
    println!("{butt:>width$}", butt=1, width=62);
    println!("{butt:>0width$}", butt=2, width=62);
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let mut port_succ = 0;
    let mut port_fail = 0;

    let port_range = port_start..port_end;

    for port_range_port in port_range {
        let hostport = format!("{}:{}", ip, port_range_port);

        if let Ok(_stream) = TcpStream::connect(hostport) {
            println!("TCP port {} open", port_range_port);
            port_succ += 1;
        } else {
            //println!("Couldn't connect on port {}.", port_range);
            port_fail += 1;
        }
    }

    println!("{} ports open", port_succ);
    println!("{} ports closed", port_fail);
}
