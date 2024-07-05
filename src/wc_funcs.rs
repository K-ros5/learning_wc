pub fn get_words_count(buf: &[u8]) -> usize {
    let file_string = String::from_utf8_lossy(buf);
    file_string.split_whitespace().count()
}

pub fn get_lines_count(buf: &[u8]) -> usize {
    buf.iter().filter(|b| **b == b'\n').count()
}

pub fn get_bytes_count(buf: &[u8]) -> usize {
    buf.len()
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
