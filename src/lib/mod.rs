use std::io::{self, BufRead, Write};

pub mod opts;
use opts::Opts;

/// Make the text from the input pretty colors
pub fn lolify<Input, Output>(input: Input, output: &mut Output, opts: &Opts) -> io::Result<()>
where
    Input: BufRead,
    Output: Write,
{
    for line in input.lines() {
        let line = line.expect("Could not read line in input stream");
        // TODO: Just print (C) for testing
        writeln!(output, "(C) {}", line)?;
    }
    Ok(())
}
