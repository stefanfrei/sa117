struct Prism {
    s: f64,
    h: f64
}

impl Prism {
    fn vol(&self) -> f64 {
        (1.0/3.0) * ( self.s * self.s ) * self.h
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vol() {
        assert_eq!(Prism{s: 2.0, h: 25.0}.vol(), 33.33333333333333);
    }
}