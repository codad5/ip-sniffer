extern crate fli;
use fli::Fli;
use std::{ net::{IpAddr, TcpStream}, str::FromStr, process, sync::mpsc::{Sender, channel}, thread, io::{self, Write}};


const MAX_PORT : u16 = 65535;
struct Arguments{
    ipaddr:IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(addr: String, threads : String) -> Result<Arguments, &'static str>
    {
        let ipaddress = match IpAddr::from_str(&addr) {
            Ok(s) => s,
            Err(_) =>  return Err("Invalid IpAddress"),
        };
        let threads = match threads.parse::<u16>() {
            Ok(thread) => thread,
            Err(_) => return Err("Invalid thread")
        };
        return Ok(Arguments{ipaddr : ipaddress, threads});
    }
}


fn main() {
    let mut app = Fli::init("sniffer", "an app to sniff port");
    app.command("snif", "snif default");
    app.option("-ip --ipaddress, <>","to set ipaddress", snif);
    app.option("-th --thread, []", "to set thread number default 4", |_x|{});
    app.run();
}

fn snif(x: &Fli)
{
    let ipaddress = match x.get_values("--ipaddress".to_owned()){
        Ok(values) =>  values[0].to_string(),
        Err(_) => "".to_string(),
    };
    let thread_num = match x.get_values("--thread".to_owned()){
            Ok(values) => values[0].to_string(),
            Err(_) => "4".to_string()
    };
    let arguments = match Arguments::new(ipaddress, thread_num) {
        Ok(arg) =>  arg,
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
    };
    let thread_num = arguments.threads;
    let ipaddress = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..thread_num {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, ipaddress , thread_num);
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx{
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out{
        println!("{} is open ", v);
    }
}


fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr,  num_thread : u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {
                continue;
            }
        };
        if (MAX_PORT - port) < num_thread {
            break;
        }
        port += num_thread;
    }
}
