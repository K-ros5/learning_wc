use clap::Parser;
use learning_wc::{get_bytes_count, get_lines_count, get_words_count};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

///Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified. With no FILE, or when FILE is -, read standard input.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///print the byte counts
    #[arg(
        short = 'c',
        long,
        num_args(0),
        require_equals(false),
        default_missing_value("true")
    )]
    bytes: bool,

    //print the newline counts
    #[arg(
        short,
        long,
        num_args(0),
        require_equals(false),
        default_missing_value("true")
    )]
    lines: bool,

    //print the word counts
    #[arg(
        short,
        long,
        num_args(0),
        require_equals(false),
        default_missing_value("true")
    )]
    words: bool,

    ///optional files
    file: Option<Vec<PathBuf>>,
}

//Both File and Stdin implement Read
fn process_input(input: &mut impl Read, bytes: bool, lines: bool, words: bool) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    let mut buf: Vec<u8> = Vec::new();
    input.read_to_end(&mut buf).expect("File read error!");

    let no_args = !bytes && !lines && !words;

    if bytes || no_args {
        output.push(format!("{}", get_bytes_count(&buf)));
    }

    if lines || no_args {
        output.push(format!("{}", get_lines_count(&buf)));
    }

    if words || no_args {
        output.push(format!("{}", get_words_count(&buf)));
    }

    output
}

fn main() {
    let cli = Args::parse();

    if cli.file.is_some() {
        cli.file.inspect(|path| {
            let path: PathBuf = path.iter().collect();
            let mut file = File::open(&path);

            if let Ok(ref mut file) = &mut file {
                let output = process_input(file, cli.bytes, cli.lines, cli.words);
                println!("{} {}", output.join(" "), &path.to_string_lossy())
            } else {
                println!("{}: Error opening file", &path.to_string_lossy())
            }
        });
    } else {
        let mut stdin = std::io::stdin();
        let output = process_input(&mut stdin, cli.bytes, cli.lines, cli.words);
        println!("{}", output.join(" "));
    }
}
