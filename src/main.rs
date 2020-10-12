use std::env;
use failure::Error;
use std::io::{self, Read};
use atty::Stream;
use std::io::{stdout, Write, BufWriter};

// エイリアス
pub type Result<T> = std::result::Result<T, Error>;

fn is_pipe() -> bool {
    // Terminalでなければ標準入力から読み込む
    ! atty::is(Stream::Stdin)
}

fn read_from_stdin() -> Result<String> {
    // パイプで渡されていなければヘルプ表示
    if ! is_pipe() {
        // 引数がなければヘルプ表示
        panic!("required stdin from pipeline.");
    }

    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

fn separate_options(args: Vec<String>) -> Result<Vec<Vec<String>>> {
    let mut is_v:bool = false;
    let mut is_force_v:bool = false;
    let mut excludes = Vec::<String>::new();
    let mut includes = Vec::<String>::new();

    for arg in args {
        let arg_str: &str = arg.as_str();

        match arg_str {
            "-v" => is_v = true,
            "-V" => is_force_v = true,
            "-G" => is_force_v = false,
            _ => {
                if is_v || is_force_v {
                    excludes.push(arg_str.to_string());
                } else {
                    includes.push(arg_str.to_string());
                }

                is_v = false;
            }
        }
    }
    Ok(vec![includes, excludes])
}

fn is_included(input: std::string::String, targets: Vec<String>) -> Result<bool> {
    for target in targets {
        if !input.contains(&target) { return Ok(false) }
    }

    Ok(true)
}

fn is_excluded(input: std::string::String, targets: Vec<String>) -> Result<bool> {
    for target in targets {
        if input.contains(&target) { return Ok(false) }
    }

    Ok(true)
}

fn extract(input: std::string::String, includes: Vec<String>, excludes: Vec<String>) -> Result<Vec<String>> {
    let mut remained_lines = Vec::<String>::new();

    for line in input.lines() {
        if is_included(line.to_string(), includes.clone())? && is_excluded(line.to_string(), excludes.clone())?{
            remained_lines.push(line.to_string());
        }
    }
    Ok(remained_lines)
}



fn output(lines: Vec<String>) -> Result<()> {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    for line in lines {
        writeln!(out, "{}", line).unwrap();
    }

    Ok(())
}

fn main() -> Result<()>{
    let targets: String = read_from_stdin()?;

    let mut args: Vec<String> = env::args().collect();
    //コマンド名部分は不要なので削除
    args.remove(0);
    let separated_options = separate_options(args)?;
    let includes = &separated_options[0];
    let excludes = &separated_options[1];

    let extracted_lines: Vec<String> = extract(targets, includes.to_vec(), excludes.to_vec())?;

    output(extracted_lines)?;

    Ok(())
}

