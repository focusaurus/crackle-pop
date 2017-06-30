#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_snap() {
        assert_eq!(snap(0), "CracklePop");
        assert_eq!(snap(1), "1");
        assert_eq!(snap(2), "2");
        assert_eq!(snap(3), "Crackle");
        assert_eq!(snap(4), "4");
        assert_eq!(snap(5), "Pop");
        assert_eq!(snap(15), "CracklePop");
        assert_eq!(snap(45), "CracklePop");
        assert_eq!(snap(33333), "Crackle");
        assert_eq!(snap(55555), "Pop");
        assert_eq!(snap(55558), "55558");
    }
}

pub fn snap(number: u32) -> String {
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
