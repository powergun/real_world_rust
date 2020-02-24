
pub mod filepath;

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_file_return_ok() {
        let _ = super::filepath::read_file_ok("testdata/search_replace.txt");
    }

    #[test]
    fn test_read_file_return_string() {
        let result = super::filepath::read_file("testdata/search_replace.txt");
        match result {
            Ok(v) => {
                assert_eq!("there is a cow", v)
            }
            Err(_) => {
                panic!("failed!")
            }
        }
    }
}
