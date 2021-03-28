struct Circle {
    dia: f64
}

impl Circle {
    fn area(&self) -> f64 {
        self.dia * std::f64::consts::PI
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(Circle{dia: 40.0}.area(), 125.66370614359172);
    }

}
