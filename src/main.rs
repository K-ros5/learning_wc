use clap::Parser;
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

fn get_words_count(buf: &[u8]) -> usize {
    let file_string = String::from_utf8_lossy(buf);
    file_string.split_whitespace().count()
}

fn get_lines_count(buf: &[u8]) -> usize {
    buf.iter().filter(|b| **b == b'\n').count()
}

fn get_bytes_count(buf: &[u8]) -> usize {
    buf.len()
}

fn main() {
    let cli = Args::parse();

    cli.file.inspect(|path| {
        let path: PathBuf = path.iter().collect();
        let mut file = File::open(&path);
        let mut output: Vec<String> = Vec::new();

        if let Ok(ref mut file) = file {
            let mut buf: Vec<u8> = Vec::new();
            file.read_to_end(&mut buf).expect("File read error!");

            let no_args = !cli.bytes && !cli.lines && !cli.words;

            if cli.bytes || no_args {
                output.push(format!("{}", get_bytes_count(&buf)));
            }

            if cli.lines || no_args {
                output.push(format!("{}", get_lines_count(&buf)));
            }

            if cli.words || no_args {
                output.push(format!("{}", get_words_count(&buf)));
            }
        } else {
            println!("{}: Error opening file", &path.to_string_lossy())
        }

        println!("{} {}", output.join(" "), &path.to_string_lossy())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bytes_count_returns_valid_count() {
        let mut byte_buf: Vec<u8> = vec![1, 2, 3, 4, 5];
        assert_eq!(get_bytes_count(&byte_buf), 5);

        //check for zero length
        byte_buf.clear();
        assert_eq!(get_bytes_count(&byte_buf), 0);
    }

    #[test]
    fn get_lines_count_valid_count() {
        let mut byte_buf: Vec<u8> = vec![b'\n'; 5];
        assert_eq!(get_lines_count(&byte_buf), 5);

        //check for zero length
        byte_buf.clear();
        assert_eq!(get_lines_count(&byte_buf), 0);
    }

    #[test]
    fn get_words_count_valid_count() {
        let mut words = String::from(" WORD ");
        words = words.repeat(5);

        let mut byte_buf: Vec<u8> = words.into_bytes();

        assert_eq!(get_words_count(&byte_buf), 5);

        //check for zero length
        byte_buf.clear();
        assert_eq!(get_words_count(&byte_buf), 0);
    }
}
