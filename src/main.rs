use std::fs::OpenOptions;
use std::io::Write;
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

    /// Write RTT messages to file
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let address = format!("{}:{}", args.host, args.port);
    let output = args.output;
    let mut waiting = false;

    let should_write;
    let file_writer;

    match output {
        Some(output) => {
            file_writer = OpenOptions::new()
                .create(true)
                .append(true) // append(true) implies write(true)
                .open(output);

            match file_writer {
                Ok(_) => {
                    should_write = true;
                }
                Err(e) => {
                    eprintln!("{} {}", "error:".bright_red().bold(), e);
                    should_write = false;
                }
            };
        }
        None => {
            should_write = false;
        }
    };

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
                    let time_now = Local::now().format("%T%.3f");
                    if should_write {
                        // Cannot be uninitialized as the file flag is only true if file was successfully opened, so why does it complain?
                        match writeln!(file_writer.unwrap(), "[{}] {}", &time_now, &line) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("{} {}", "error:".bright_red().bold(), e);
                            }
                        }
                    }
                    println!(
                        "{}",
                        format_args!("{} {}", format!("[{}]", time_now).bright_black(), line)
                    );
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
