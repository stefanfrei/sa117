struct Sphere {
    dia: f64
}

impl Sphere {
    fn vol(&self) -> f64 {
        1.33333 * std::f64::consts::PI * ( (self.dia / 2.0 ) * (self.dia / 2.0 ) * (self.dia / 2.0 ) )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vol() {
        assert_eq!(Sphere{dia: 34.0}.vol(), 20579.474827299848);
    }
}
