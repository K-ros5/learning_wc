#[cfg(test)]
mod tests {
    use std::process::Command;

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
}
