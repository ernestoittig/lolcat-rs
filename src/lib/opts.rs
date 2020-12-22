use clap::{crate_description, crate_version, AppSettings, Clap, IntoApp};
use rand::{thread_rng, Rng};
use std::io::{self, Write};

#[derive(Clap, PartialEq, Debug)]
#[clap(
    version = crate_version!(),
    about = crate_description!(),
    // TODO: This might change in a future version of clap
    setting = AppSettings::NoAutoHelp,
)]
pub struct Opts {
    #[clap(
        about = "Concatenate file(s), or standard input, to standard output.\n\
        With no file, or when file is -, read standard input."
    )]
    pub files: Vec<String>,
    #[clap(
        long,
        short = 'p',
        value_name = "f",
        about = "Rainbow spread",
        default_value = "3.0"
    )]
    pub spread: f32,
    #[clap(
        long,
        short = 'F',
        value_name = "f",
        about = "Rainbow frequency",
        default_value = "0.1"
    )]
    pub freq: f32,
    #[clap(
        long,
        short = 'S',
        value_name = "i",
        about = "Rainbow seed, 0 = random",
        default_value = "0"
    )]
    pub seed: u32,
    #[clap(long, short, about = "Enable psychedelics")]
    pub animate: bool,
    #[clap(
        long,
        short,
        value_name = "i",
        about = "Animation duration",
        default_value = "12"
    )]
    pub duration: u32,
    #[clap(
        long,
        short,
        value_name = "f",
        about = "Animation speed",
        default_value = "20.0"
    )]
    pub speed: f32,
    #[clap(long, short, about = "Invert fg and bg")]
    pub invert: bool,
    #[clap(long, short, about = "24-bit truecolor")]
    pub truecolor: bool,
    #[clap(
        long,
        short,
        about = "Force color even when stdocolor even when stdout is not a ttyut is not a tty"
    )]
    pub force: bool,

    // custom flags normally set by clap
    #[clap(long, short, about = "Show this message")]
    pub help: bool,
    #[clap(long, short, about = "Print version then exit")]
    pub version: bool,
}

impl Opts {
    pub fn defaults() -> Self {
        let empty: [String; 0] = [];
        Self::parse_from(&empty)
    }

    /// Sets the seed to a random number if zero and populates files if empty
    pub fn normalize(&mut self) {
        if self.seed == 0 {
            self.seed = thread_rng().gen_range(0..256);
        }
        if self.files.is_empty() {
            self.files.push("-".to_owned());
        }
    }

    /// Writes the help text to a buffer
    pub fn help_text<T: Write>(buffer: &mut T) -> io::Result<()> {
        <Self as IntoApp>::into_app().write_help(buffer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::crate_name;

    #[test]
    fn test_defaults() {
        let defaults = Opts {
            files: vec![],
            spread: 3.0,
            freq: 0.1,
            seed: 0,
            animate: false,
            duration: 12,
            speed: 20.0,
            invert: false,
            truecolor: false,
            force: false,
            help: false,
            version: false,
        };
        let generated = Opts::defaults();
        assert_eq!(generated, defaults);
    }

    #[test]
    fn test_help_message() -> io::Result<()> {
        let mut bytes = vec![0u8; 0];
        Opts::help_text(&mut bytes)?;
        let string = String::from_utf8(bytes).expect("Invalid help string");
        assert!(string.starts_with(&format!("{} {}", crate_name!(), crate_version!())));
        Ok(())
    }

    #[test]
    fn test_normal_long_flags() {
        let target = Opts {
            files: vec!["Test file".to_owned()],
            spread: 5.0,
            freq: 0.2,
            seed: 162,
            animate: true,
            duration: 15,
            speed: 30.0,
            invert: true,
            truecolor: true,
            force: true,
            // These are not normal flags
            help: false,
            version: false,
        };

        let generated = Opts::parse_from(&[
            crate_name!(),
            "--spread",
            "5",
            "--freq",
            "0.2",
            "--seed",
            "162",
            "--animate",
            "--duration",
            "15",
            "--speed",
            "30",
            "--invert",
            "--truecolor",
            "--force",
            "Test file",
        ]);

        assert_eq!(generated, target);
    }

    #[test]
    fn test_normal_short_flags() {
        let target = Opts {
            files: vec!["Test file".to_owned()],
            spread: 5.0,
            freq: 0.2,
            seed: 162,
            animate: true,
            duration: 15,
            speed: 30.0,
            invert: true,
            truecolor: true,
            force: true,
            // These are not normal flags
            help: false,
            version: false,
        };
        let generated = Opts::parse_from(&[
            crate_name!(),
            "-p",
            "5",
            "-F",
            "0.2",
            "-S",
            "162",
            "-a",
            "-d",
            "15",
            "-s",
            "30",
            "-i",
            "-t",
            "-f",
            "Test file",
        ]);
        assert_eq!(generated, target);
    }

    #[test]
    fn test_help_flag() {
        let generated_long = Opts::parse_from(&[crate_name!(), "--help"]);
        assert!(generated_long.help);
        let generated_short = Opts::parse_from(&[crate_name!(), "-h"]);
        assert!(generated_short.help);
    }
}
