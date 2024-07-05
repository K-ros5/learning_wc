#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::Write,
        process::{Command, Stdio},
    };

    #[test]
    fn byte_count() {
        let response = Command::new("./target/debug/learning_wc")
            .arg("--bytes")
            .arg("./test_files/test.txt")
            .output()
            .expect("Binary not found?")
            .stdout;

        let command_string = String::from_utf8(response).unwrap();

        assert_eq!(command_string, "342190 ./test_files/test.txt\n");
    }

    #[test]
    fn line_count() {
        let response = Command::new("./target/debug/learning_wc")
            .arg("--lines")
            .arg("./test_files/test.txt")
            .output()
            .expect("Binary not found?")
            .stdout;

        let command_string = String::from_utf8(response).unwrap();

        assert_eq!(command_string, "7145 ./test_files/test.txt\n");
    }

    #[test]
    fn word_count() {
        let response = Command::new("./target/debug/learning_wc")
            .arg("--words")
            .arg("./test_files/test.txt")
            .output()
            .expect("Binary not found?")
            .stdout;

        let command_string = String::from_utf8(response).unwrap();

        assert_eq!(command_string, "58164 ./test_files/test.txt\n");
    }

    #[test]
    fn check_stdin() {
        let mut child = Command::new("./target/debug/learning_wc")
            .arg("-c")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Binary not found?");

        let mut stdin = child.stdin.take().expect("stdin failed");
        std::thread::spawn(move || {
            stdin
                .write_all(
                    fs::read_to_string("./test_files/test.txt")
                        .expect("Couldn't read test file")
                        .as_bytes(),
                )
                .expect("Failed to write to stdin");
        });

        let response = child.wait_with_output().expect("Failed to read stdout");

        let command_string = String::from_utf8(response.stdout).unwrap();

        assert_eq!(command_string, "342190\n");
    }

    #[test]
    fn chars_count() {
        let response = Command::new("./target/debug/learning_wc")
            .arg("--chars")
            .arg("./test_files/test.txt")
            .output()
            .expect("Binary not found?")
            .stdout;

        let command_string = String::from_utf8(response).unwrap();

        assert_eq!(command_string, "339292 ./test_files/test.txt\n");
    }

    #[test]
    fn no_options_but_file() {
        let response = Command::new("./target/debug/learning_wc")
            .arg("./test_files/test.txt")
            .output()
            .expect("Binary not found?")
            .stdout;

        let command_string = String::from_utf8(response).unwrap();

        assert_eq!(command_string, "7145 58164 342190 ./test_files/test.txt\n");
    }
}
