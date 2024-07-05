use clap::Parser;
use learning_wc::{get_bytes_count, get_characters_count, get_lines_count, get_words_count};
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

    ///print the newline counts
    #[arg(
        short,
        long,
        num_args(0),
        require_equals(false),
        default_missing_value("true")
    )]
    lines: bool,

    ///print the word counts
    #[arg(
        short,
        long,
        num_args(0),
        require_equals(false),
        default_missing_value("true")
    )]
    words: bool,

    ///print the character counts
    #[arg(
        short = 'm',
        long,
        num_args(0),
        require_equals(false),
        default_missing_value("true")
    )]
    chars: bool,

    ///optional files, if no files takes stdin input
    file: Option<Vec<PathBuf>>,
}

//Both File and Stdin implement Read
fn process_input(input: &mut impl Read, cli: &Args) -> String {
    let mut output: Vec<String> = Vec::new();

    let mut buf: Vec<u8> = Vec::new();
    input.read_to_end(&mut buf).expect("File read error!");

    let no_args = !cli.bytes && !cli.lines && !cli.words && !cli.chars;

    //wc output with no args is "{lines} {words} {bytes}"
    if cli.lines || no_args {
        output.push(get_lines_count(&buf).to_string());
    }

    if cli.words || no_args {
        output.push(get_words_count(&buf).to_string());
    }

    if cli.bytes || no_args {
        output.push(get_bytes_count(&buf).to_string());
    }

    if cli.chars {
        output.push(get_characters_count(&buf).to_string());
    }

    output.join(" ")
}

fn main() {
    let cli = Args::parse();
    let no_files = cli.file.is_none();

    for path in cli.file.iter().flatten() {
        let mut file = File::open(path);

        if let Ok(ref mut file) = &mut file {
            let output = process_input(file, &cli);
            println!("{} {}", output, &path.to_string_lossy())
        } else {
            println!("{}: Error opening file", &path.to_string_lossy())
        }
    }

    if no_files {
        let mut stdin = std::io::stdin();
        let output = process_input(&mut stdin, &cli);
        println!("{}", output);
    }
}
