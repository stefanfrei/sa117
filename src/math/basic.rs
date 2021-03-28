fn add(a: u8, b: u8) -> u8 {
    a + b
}

fn subtract(a: u8, b: u8) -> u8 {
    a - b
}

fn multiply(a: u8, b: u8) -> u8 {
    a * b
}

fn divide(a: u8, b: u8) -> u8 {
    a / b
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(110, 22), 132);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(3, 2), 1);
        assert_eq!(subtract(110, 20), 90);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(1, 2), 2);
        assert_eq!(multiply(12, 12), 144);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(9, 3), 3);
        assert_eq!(divide(120, 30), 4);
        assert_eq!(divide(120, 3), 40);
        assert_eq!(divide(121, 3), 40);
    }

}