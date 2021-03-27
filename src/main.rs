fn main() {
    println!("Hello, world!");
}

fn add(a: u8, b: u8) -> i16 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
fn subtract(a: u32, b: u32) -> u64 {
    (a - b).into()
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(250, 22), 272);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(1, 2), 3);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_divide() {
        assert_eq!(subtract(1, 2), 3);
    }
}
