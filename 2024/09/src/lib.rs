const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

#[derive(Debug, PartialEq)]
pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl From<SnowKg> for Snowball {
    // 1. Implement the conversion from SnowKg to Snowball
    fn from(SnowKg(kg): SnowKg) -> Self {
        let snowballs_float = kg / SNOWBALL_WEIGHT_KG;
        Self(snowballs_float.round() as i64)
    }
}

// 2. Implement the same for SnowLb
impl From<SnowLb> for Snowball {
    fn from(SnowLb(lb): SnowLb) -> Self {
        let snowballs_float = lb / SNOWBALL_WEIGHT_LB;
        Self(snowballs_float.round() as i64)
    }
}

#[cfg(test)]
mod test {
    use crate::{SnowKg, SnowLb, Snowball};

    #[test]
    fn test_kg() {
        assert_eq!(Snowball::from(SnowKg(0.49)), Snowball(2));
        assert_eq!(Snowball::from(SnowKg(0.5)), Snowball(3));
        assert_eq!(Snowball::from(SnowKg(0.51)), Snowball(3));
    }

    #[test]
    fn test_lb() {
        assert_eq!(Snowball::from(SnowLb(1.102)), Snowball(2));
        assert_eq!(Snowball::from(SnowLb(1.1025)), Snowball(3));
        assert_eq!(Snowball::from(SnowLb(1.103)), Snowball(3));
    }
}
