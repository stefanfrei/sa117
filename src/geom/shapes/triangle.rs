struct Triangle {
    a: f64,
    b: f64,
    #[allow(dead_code)]
    c: f64
}

impl Triangle {

    fn area(&self) -> f64 {
        self.a * self.b / 2.0
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(Triangle{a: 2.0, b: 2.0, c: 2.828}.area(), 2.0);
        assert_eq!(Triangle{a: 27.5, b: 8.0, c: 28.64}.area(), 110.0);
    }
}