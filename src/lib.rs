#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format() {
        assert_eq!(format(0), "CracklePop");
        assert_eq!(format(1), "1");
        assert_eq!(format(2), "2");
        assert_eq!(format(3), "Crackle");
        assert_eq!(format(4), "4");
        assert_eq!(format(5), "Pop");
        assert_eq!(format(15), "CracklePop");
        assert_eq!(format(45), "CracklePop");
        assert_eq!(format(33333), "Crackle");
        assert_eq!(format(55555), "Pop");
        assert_eq!(format(55558), "55558");
    }
}

pub fn format(number: u32) -> String {
    let mut result = String::new();
    let by3 = number % 3 == 0;
    let by5 = number % 5 == 0;
    let neither = !by3 && !by5;
    if by3 {
        result.push_str("Crackle");
    }
    if by5 {
        result.push_str("Pop");
    }
    if neither {
        result.push_str(format!("{}", number).as_str());
    }
    result
}
