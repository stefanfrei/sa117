struct Dice {
    s: f64
}

impl Dice {
    fn vol(&self) -> f64 {
        self.s * self.s * self.s
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vol() {
        assert_eq!(Dice{s: 40.0}.vol(), 64000.0);
    }
}
