use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]

/// Rust head
#[command(version, about,long_about=None)]
pub struct Arg {
    /// Input file(s)
    #[arg(value_name = "FILE",default_value = "-", num_args(1..))]
    files: Vec<String>,
    /// Number of lines
    #[arg(value_name = "LINES", short = 'n', long, default_value_t = 10)]
    lines: usize,
    /// Number of bytes
    #[arg(value_name = "BYTES", short = 'c', long, conflicts_with("lines"))]
    bytes: Option<usize>,
}
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn run() -> MyResult<()> {
    let args = Arg::parse();
    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" },);
                }

                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        }
    }

    Ok(())
}
