#[derive(Copy, Clone)]
pub struct Percentage {
    value: f64,
}

qty_ctor! {
    Percentage => {
        (from_percentage, as_percentage, "%", 1.0),
        (from_fraction, as_fraction, None, 100.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage() {
        let p = Percentage::from_percentage(0.05342);
        assert_eq!(p.as_percentage(), 0.05342);
        assert_eq!(p.as_fraction(), 0.0005342);
        assert_eq!(format!("{}", p), "0.053 %");
        assert_eq!(format!("{:.2}", p), "0.05 %");
        assert_eq!(format!("{:.8}", p), "0.05342 %");

        let p = Percentage::from_percentage(50.0);
        assert_eq!(p.as_percentage(), 50.0);
        assert_eq!(p.as_fraction(), 0.5);
        assert_eq!(format!("{}", p), "50 %");

        let p = Percentage::from_fraction(0.75);
        assert_eq!(p.as_percentage(), 75.0);
        assert_eq!(p.as_fraction(), 0.75);

        let p = Percentage::from_fraction(350.0);
        assert_eq!(p.as_percentage(), 35000.0);
        assert_eq!(p.as_fraction(), 350.0);
        assert_eq!(format!("{}", p), "35000 %");
    }
}
