use std::{env, net::{IpAddr, TcpStream, SocketAddr}, str::FromStr, process, sync::mpsc::{Sender, channel}, thread, io::{self, Write}};


const MAX_PORT : u16 = 65535;
struct Arguments{
    flag :String,
    ipaddr:IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> 
    {
        if args.len() < 2 {
            return Err("Not enough argument");
        }
        if args.len() > 4 {
            return Err("Too many arguments");
        }
        let mut flag =  args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            return Ok(Arguments{flag : String::from(""), ipaddr, threads: 4});
        }
        else{
            flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Help statement");
                return Err("help");
            }
            if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) =>  return Err("Invalid IpAddress"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(thread) => thread,
                    Err(_) => return Err("Invalid thread")
                };
                return Ok(Arguments{flag, ipaddr, threads});
            }
        }
        return Err("An Error Occcured : Failed to init struct")
    }
}


fn main() {
   let args : Vec<String> = env::args().collect();
    println!("Hello, world!, {:?}", args);
    run_callback(make);
    let arguments = match Arguments::new(&args) {
        Ok(arg) => arg,
        Err(err) => {
            println!("{}", err);
            if err.contains("help") {

                process::exit(0)
            }
            process::exit(1)
        }
    };

    let num_thread = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_thread {
        let tx = tx.clone();
        thread::spawn(move || {
            print!("Running \n");
            scan(tx, i, addr , num_thread);
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
        println!("{} is opne ", v);
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr,  num_thread : u16) {
    let mut port = start_port + 1;
    println!("No of thread {}", num_thread);
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!(". Port : {}", port);
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
        println!("Port+thread = {}", port);
    }
}



fn run_callback(callback : fn())
{
    callback();
}