#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format() {
        assert_eq!(format(0), "0");
    }
}

pub fn format(number: u32) -> String {
    String::from("")
}
