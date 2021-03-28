struct Rect {
    a: f64,
    b: f64
}

impl Rect {
   fn area(&self) -> f64 {
       self.a * self.b
   } 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(Rect{a: 2.0, b: 2.0}.area(), 4.0);
        assert_eq!(Rect{a: 27.5, b: 8.0}.area(), 220.0);
    }
}