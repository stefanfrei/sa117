struct Cube {
    a: f64,
    b: f64,
    c: f64
}

impl Cube {
    fn vol(&self) -> f64 {
        self.a * self.b * self.c
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vol() {
        assert_eq!(Cube{a: 1.0, b: 1.0, c: 1.0}.vol(), 1.0);
    }
}