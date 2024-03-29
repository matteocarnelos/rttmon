use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Error;
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
    #[arg(short = 'o', long = "output")]
    path: Option<PathBuf>,
}

fn main() {
    if let Err(e) = rttmon() {
        eprintln!("{} {}", "error:".bright_red().bold(), e);
        exit(1);
    }
}

fn rttmon() -> Result<(), Error> {
    let args = Args::parse();
    let address = format!("{}:{}", args.host, args.port);
    let path = args.path;
    let mut waiting = false;
    let mut file = if let Some(p) = path {
        Some(OpenOptions::new().create(true).write(true).open(p)?)
    } else {
        None
    };

    loop {
        sleep(Duration::from_secs(1));
        let stream = match TcpStream::connect(&address) {
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
                }
                return Err(e.into());
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
                    let now = Local::now().format("%T%.3f");
                    if let Some(f) = &mut file {
                        write!(f, "[{}] {}", now, line)?;
                    }
                    print!(
                        "{}",
                        format_args!("{} {}", format!("[{}]", now).bright_black(), line)
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
