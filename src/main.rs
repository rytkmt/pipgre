use std::env;
use std::io::{self, Read, Write, BufWriter, stdout};
use std::process;
use anyhow::Result;
use atty::Stream;

struct FilterOptions {
    includes: Vec<String>,
    excludes: Vec<String>,
}

fn read_from_stdin() -> Result<String> {
    if atty::is(Stream::Stdin) {
        eprintln!("Usage: <command> | pipgre [options] [patterns]");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  <pattern>    Include lines containing pattern (AND)");
        eprintln!("  -v <pattern> Exclude lines containing the next pattern");
        eprintln!("  -V           Exclude mode: all following patterns are excludes");
        eprintln!("  -G           Include mode: cancel -V, following patterns are includes");
        process::exit(1);
    }

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;
    Ok(buf)
}

fn parse_options(args: Vec<String>) -> FilterOptions {
    let mut exclude_mode = false;
    let mut next_is_exclude = false;
    let mut includes = Vec::new();
    let mut excludes = Vec::new();

    for arg in args {
        match arg.as_str() {
            "-v" => next_is_exclude = true,
            "-V" => {
                exclude_mode = true;
                next_is_exclude = false;
            }
            "-G" => {
                exclude_mode = false;
                next_is_exclude = false;
            }
            _ => {
                if next_is_exclude || exclude_mode {
                    excludes.push(arg);
                } else {
                    includes.push(arg);
                }
                next_is_exclude = false;
            }
        }
    }

    FilterOptions { includes, excludes }
}

fn matches_all(line: &str, patterns: &[String]) -> bool {
    patterns.iter().all(|p| line.contains(p.as_str()))
}

fn matches_none(line: &str, patterns: &[String]) -> bool {
    patterns.iter().all(|p| !line.contains(p.as_str()))
}

fn extract<'a>(input: &'a str, options: &FilterOptions) -> Vec<&'a str> {
    input
        .lines()
        .filter(|line| matches_all(line, &options.includes) && matches_none(line, &options.excludes))
        .collect()
}

fn main() -> Result<()> {
    let input = read_from_stdin()?;

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let options = parse_options(args);

    let lines = extract(&input, &options);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for line in lines {
        writeln!(out, "{}", line)?;
    }

    Ok(())
}