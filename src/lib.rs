#[macro_use]
extern crate uom;

ISQ!(
    uom::si,
    f64,
    (foot, pound, second, ampere, degree_rankine, mole, candela)
);

#[cfg(test)]
mod tests {
    #[test]
    fn equal() {
        let value = super::Length::new::<uom::si::length::foot>(28.0);

        assert_eq!(value, value);
    }
}
