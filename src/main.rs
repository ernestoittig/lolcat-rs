use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

mod lib;
use clap::Clap;
use lib::lolify;
use lib::opts::Opts;

use atty::Stream;

fn main() -> io::Result<()> {
    let mut opts: Opts = Opts::parse();
    let print_colors = opts.force || atty::is(Stream::Stdout);

    if opts.help {
        let mut help_text: Vec<u8> = vec![];
        Opts::help_text(&mut help_text)?;
        let mut new_opts = Opts::defaults();
        new_opts.normalize();
        if print_colors {
            lolify(&help_text[..], &mut io::stdout(), &new_opts)?;
        } else {
            let mut stdout = io::stdout();
            stdout.write_all(&help_text)?;
        }
        std::process::exit(0);
    }

    opts.normalize();

    for filename in &opts.files {
        if filename == "-" {
            let stdin = io::stdin();
            let lock = stdin.lock();

            if print_colors {
                lolify(lock, &mut io::stdout(), &opts)?;
            } else {
                for line in lock.lines() {
                    let line = line.expect("Could not read line from stdin");
                    println!("{}", line);
                }
            }
        } else {
            let file = OpenOptions::new().read(true).open(filename)?;
            let reader = BufReader::new(file);

            if print_colors {
                lolify(reader, &mut io::stdout(), &opts)?;
            } else {
                for line in reader.lines() {
                    let line = line.expect(&format!("Could not read line from file {}", filename));
                    println!("{}", line);
                }
            }
        }
    }

    Ok(())
}
