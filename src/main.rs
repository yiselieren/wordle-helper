// Wordle puzzle helper

use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

const EMPTY_STRING: String = String::new();

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "data"]
#[prefix = ""]
struct Asset;

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "wordle-helper", about = "Helps to solve the wordle puzzle")]
struct Opt {
    /// Verbose mode
    #[structopt(short, long)]
    verbose: bool,

    /// Debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Even more debug mode
    #[structopt(short = "D", long = "DEBUG")]
    more_debug: bool,

    /// Set word length
    #[structopt(short = "l", long = "length", default_value = "5")]
    wlen: usize,

    /// Small amount of lines to print without "less"
    #[structopt(short, long, default_value = "15")]
    small: usize,
}

fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}

fn enter_uniq_string(n: &str, s: String) -> String {
    let mut reply: String = String::new();
    let mut rc: String = s;

    println!("\nThe current {} are: \"{}\":", &n, &rc);
    print!("  Enter {} characters <Enter> if finished: ", &n);
    std::io::stdout().flush().unwrap();
    reply.clear();
    io::stdin()
        .read_line(&mut reply)
        .expect("Failed to read line");
    trim_newline(&mut reply);
    if reply.is_empty() {
        return rc;
    }
    rc.clear();
    for c in reply.chars() {
        if !rc.contains(c) {
            rc.push(c);
        }
    }

    rc
}

fn enter_uniq_string_single_pair(n: &str, wlen: usize, s: Vec<char>) -> Vec<char> {
    let mut rc: Vec<char> = s;
    let mut reply: String = String::new();

    loop {
        println!("\nThe current {} content is:", &n);
        for (i, e) in rc.iter().enumerate() {
            if *e == '\0' {
                println!("  {}: None", i + 1);
            } else {
                println!("  {}: \"{}\"", i + 1, e);
            }
        }
        print!(
            "  Enter {} characters (in a NUMBER CHAR fomat), <Enter> if finished: ",
            &n
        );
        std::io::stdout().flush().unwrap();
        reply.clear();
        io::stdin()
            .read_line(&mut reply)
            .expect("Failed to read line");
        trim_newline(&mut reply);
        if reply.is_empty() {
            break;
        }
        let fields: Vec<&str> = reply.split(' ').collect();
        if fields.len() != 2 {
            println!("Invalid input, should be NUMBER CHARACTER");
            continue;
        }
        let n: usize = match fields[0].parse() {
            Err(_) => {
                println!("\"{}\" is invalid NUMBER", fields[0]);
                continue;
            }
            Ok(n) => n,
        };
        if n < 1 || n > wlen {
            println!(
                "\"{}\" is invalid NUMBER, should be in 1..{} range",
                fields[0], wlen
            );
            continue;
        }
        if fields[1].len() != 1 {
            println!("\"{}\" is invalid CHARACTER, should be 1 char", fields[1])
        }
        rc[n - 1] = fields[1].chars().next().unwrap();
    }

    rc
}

fn enter_uniq_string_multi_pair(n: &str, wlen: usize, s: Vec<String>) -> Vec<String> {
    let mut rc: Vec<String> = s;

    let mut reply: String = String::new();

    loop {
        println!("\nThe current {} content is:", &n);
        for (i, e) in rc.iter().enumerate() {
            if e.is_empty() {
                println!("  {}: None", i + 1);
            } else {
                println!("  {}: \"{}\"", i + 1, e);
            }
        }
        print!(
            "  Enter {} characters (in a NUMBER CHARS fomat), <Enter> if finished: ",
            &n
        );
        std::io::stdout().flush().unwrap();
        reply.clear();
        io::stdin()
            .read_line(&mut reply)
            .expect("Failed to read line");
        trim_newline(&mut reply);
        if reply.is_empty() {
            break;
        }
        let fields: Vec<&str> = reply.split(' ').collect();
        if fields.len() != 2 {
            println!("Invalid input, should be NUMBER CHARACTERS");
            continue;
        }
        let n: usize = match fields[0].parse() {
            Err(_) => {
                println!("\"{}\" is invalid NUMBER", fields[0]);
                continue;
            }
            Ok(n) => n,
        };
        if n < 1 || n > wlen {
            println!(
                "\"{}\" is invalid NUMBER, should be in 1..{} range",
                fields[0], wlen
            );
            continue;
        }
        rc[n - 1] = fields[1].to_string();
    }

    rc
}

fn main() {
    let opt = Opt::from_args();
    let wlen: usize = opt.wlen;
    let verbose: bool = opt.verbose;
    let debug: bool = opt.debug;
    let more_debug: bool = opt.more_debug;
    let small_amount: usize = opt.small;

    let mut not_in_word: String = EMPTY_STRING;
    let mut not_in_place: Vec<String> = vec![EMPTY_STRING; wlen];
    let mut in_place: Vec<char> = vec!['\0'; wlen];

    let words_file = Asset::get("words_alpha.txt").unwrap();
    let lines: Vec<String> = std::str::from_utf8(words_file.data.as_ref())
        .unwrap()
        .to_string()
        .split('\n')
        .filter(|&x| x.len() == wlen)
        .map(|x| x.to_string())
        .collect();

    if debug || more_debug {
        println!("{} words found", lines.len());
        if more_debug {
            let mut i: usize = 1;
            for l in &lines {
                println!(" {}: \"{}\"", i, l);
                i += 1;
            }
        }
    }

    loop {
        let mut reply: String = String::new();

        not_in_word = enter_uniq_string("NOT IN WORD", not_in_word);
        not_in_place = enter_uniq_string_multi_pair("NOT IN PLACE", wlen, not_in_place);
        in_place = enter_uniq_string_single_pair("IN PLACE", wlen, in_place);

        if verbose {
            println!("\n---\nNot in word chars are: \"{}\"", not_in_word);
            println!("Not in place chars are:");
            for (i, e) in not_in_place.iter().enumerate() {
                if e.is_empty() {
                    println!("  {}: None", i + 1);
                } else {
                    println!("  {}: \"{}\"", i + 1, e);
                }
            }
            println!("In place chars are:");
            for (i, e) in in_place.iter().enumerate() {
                if *e == '\0' {
                    println!("  {}: None", i + 1);
                } else {
                    println!("  {}: \"{}\"", i + 1, e);
                }
            }
        }

        // Result
        let mut show_lines: Vec<String> = Vec::new();
        for line in &lines {
            let mut not_show: bool = false;

            // IN PLACE
            for (i, e) in in_place.iter().enumerate() {
                if *e != '\0' && line.chars().nth(i).unwrap() != *e {
                    not_show = true;
                    break;
                }
            }
            if not_show {
                continue;
            }

            // NOT IN PLACE
            for (i, e) in not_in_place.iter().enumerate() {
                let s = &e;
                if !s.is_empty() {
                    for c in s.chars() {
                        if line.chars().nth(i).unwrap() == c {
                            not_show = true;
                            break;
                        }
                        if !line.contains(c) {
                            not_show = true;
                            break;
                        }
                    }
                }
            }
            if not_show {
                continue;
            }

            // NOT IN WORD
            for c in not_in_word.chars() {
                if line.contains(c) {
                    not_show = true;
                    break;
                }
            }
            if not_show {
                continue;
            }

            show_lines.push(line.to_string());
        }

        let amount = show_lines.len();
        print!("{} results total. Show ? [n]: ", amount);
        std::io::stdout().flush().unwrap();
        reply.clear();
        io::stdin()
            .read_line(&mut reply)
            .expect("Failed to read line");
        trim_newline(&mut reply);
        if reply.is_empty() {
            continue;
        }
        if reply.starts_with('y') {
            if amount < small_amount {
                // Just print
                for s in show_lines {
                    println!("  {}", s);
                }
            } else {
                // Show with less
                let mut less_process = Command::new("less")
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Fail to start \"less\" command");
                for s in show_lines {
                    less_process
                        .stdin
                        .as_mut()
                        .unwrap()
                        .write_all(format!("{}\r\n", s).as_bytes())
                        .expect("Can't write to \"less\" process");
                }
                //less_stdin.close();
                less_process
                    .wait()
                    .expect("Can't wait for \"less\" process");
            }
        }

        // More?
        print!("\n----------------\nMore tries ? [n] ");
        std::io::stdout().flush().unwrap();
        reply.clear();
        io::stdin()
            .read_line(&mut reply)
            .expect("Failed to read line");
        trim_newline(&mut reply);
        if reply.starts_with('y') {
            println!("\n");
        } else {
            break;
        }
    }
}
