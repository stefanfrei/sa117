fn add(a: u8, b: u8) -> u8 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
fn subtract(a: u8, b: u8) -> u8 {
    a - b
}

fn multiply(a: u8, b: u8) -> u8 {
    a * b
}

fn divide(a: u8, b: u8) -> u8 {
    a / b
}

struct Cube {
    side: f64
}

impl Cube {
    fn vol(&self) -> f64 {
        self.side * self.side * self.side
    }
}

struct Sphere {
    dia: f64
}

impl Sphere {
    fn vol(&self) -> f64 {
        self.dia * self.dia * std::f64::consts::PI
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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

    #[test]
    fn test_cube_vol() {
        assert_eq!(Cube{side: 33.2}.vol(), 36594.36800000001);
        assert_eq!(Cube{side: 10.0}.vol(), 1000.0);
    }

    #[test]
    fn test_sphere_vol() {
        assert_eq!(Sphere{dia: 33.2}.vol(), 3462.7890864928145);
        assert_eq!(Sphere{dia: 10.0}.vol(), 314.1592653589793);
    }
}
