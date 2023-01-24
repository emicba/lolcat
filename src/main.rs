use clap::{ArgAction, CommandFactory, Parser};
use rand::Rng;
use std::f64::consts::PI;
use std::io::{self, Read, Write};
use std::path::PathBuf;

const HELP_TEMPLATE: &str = "\
{before-help}{about-with-newline}
Usage: {usage}

Concatenate FILE(s) to standard output.
With no FILE, or when FILE is -, read standard input

{options}

Examples:
{tab}{bin} foo bar    # Outputs foo bar in rainbow colors
{tab}fortune | {bin}  # Outputs a rainbow cookie
";

#[derive(Parser, Debug)]
#[command(version, help_template = HELP_TEMPLATE)]
#[command(disable_help_flag = true, disable_version_flag = true)]
struct Args {
    file: Vec<PathBuf>,

    /// Rainbow spread
    #[arg(short = 'p', long, default_value_t = 3.0)]
    spread: f64,

    /// Rainbow frequency
    #[arg(short, long, default_value_t = 0.1)]
    freq: f64,

    /// Rainbow seed, 0 for random
    #[arg(short, long, default_value_t = 0)]
    seed: u32,

    /// Print help
    #[arg(short, long, action = ArgAction::SetTrue)]
    help: bool,

    /// Print version
    #[arg(short, long, action = ArgAction::Version, value_parser = clap::value_parser!(bool))]
    version: (),
}

impl Args {
    fn get_input(&self) -> String {
        let mut buf = String::new();
        if self.file.is_empty() || self.file[0].to_str() == Some("-") {
            io::stdin().lock().read_to_string(&mut buf).unwrap();
        } else {
            for path in &self.file {
                buf.push_str(&std::fs::read_to_string(path).unwrap());
            }
        }
        buf
    }

    fn get_seed(&self) -> u32 {
        if self.seed == 0 {
            rand::thread_rng().gen()
        } else {
            self.seed
        }
    }

    fn render_help() -> String {
        <Self as CommandFactory>::command()
            .render_help()
            .to_string()
    }
}

fn rainbow(freq: f64, i: f64) -> (u8, u8, u8) {
    let red = (freq * i + 0.0).sin() * 127.0 + 128.0;
    let green = (freq * i + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
    let blue = (freq * i + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;
    (red as u8, green as u8, blue as u8)
}

fn print_rainbow(input: &str, freq: f64, spread: f64, seed: u32) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let color = rainbow(freq, f64::from(seed + i as u32) / spread);
            handle
                .write(format!("\x1b[38;2;{};{};{}m{}", color.0, color.1, color.2, c).as_bytes())
                .unwrap();
        }
        handle.write(b"\n").unwrap();
    }

    print!("\x1b[0m");
}

fn main() {
    let args = Args::parse();

    if args.help {
        let help = Args::render_help();
        print_rainbow(&help, 0.4, 5.0, rand::thread_rng().gen());
        return;
    }

    let input = args.get_input();
    let spread = args.spread;
    let freq = args.freq;
    let seed = args.get_seed();

    print_rainbow(&input, freq, spread, seed);
}
