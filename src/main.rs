use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, ErrorKind};
use std::net::TcpStream;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use chrono::Local;
use clap::Parser;
use colored::Colorize;

/// A simple RTT monitor for OpenOCD
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The OpenOCD RTT server host
    #[arg(default_value = "localhost")]
    host: String,

    /// The OpenOCD RTT server port
    #[arg(default_value_t = 9090)]
    port: u16,

    /// Dump target for OpenOCD RTT data
    #[arg(default_value = "")]
    file: String,
}

/// Format data string
fn format_data(data:String) -> std::string::String {
    let now = Local::now().format("%T%.3f");
    let formatted_string = format!("{} {}", format!("[{}]", now).bright_black(), data);
    return formatted_string;
}

/// Append to file (creates file if it does not exist)
fn append_file(path: String, data:String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true) // append(true) implies write(true)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn main() {
    let args = Args::parse();
    let address = format!("{}:{}", args.host, args.port);
    let mut waiting = false;

    loop {
        sleep(Duration::from_secs(1));
        let stream = TcpStream::connect(&address);
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                if e.kind() == ErrorKind::ConnectionRefused {
                    if !waiting {
                        println!(
                            "{}",
                            format!("Waiting connection on {}...", &address.bright_blue())
                                .bright_black()
                        );
                        waiting = true;
                    }
                    continue;
                } else {
                    eprintln!("{} {}", "error:".bright_red().bold(), e);
                    exit(1);
                }
            }
        };
        println!(
            "{:─^80}",
            format!(" {} ", "Connection Established".green()).bright_black()
        );
        waiting = false;
        let mut reader = BufReader::new(stream);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(c) => {
                    if c == 0 {
                        break;
                    }
                    let formatted_line = format_data(line);
                    if args.file.len() != 0 {
                        append_file(args.file.clone(), formatted_line.clone());
                    }
                    println!("{}", formatted_line);
                }
                Err(_) => {
                    break;
                }
            }
        }
        println!(
            "{:─^80}",
            format!(" {} ", "Connection Closed".red()).bright_black()
        );
        println!();
    }
}
