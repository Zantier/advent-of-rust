use std::{cmp::Ordering, error::Error, ops::Deref};

#[derive(PartialEq)]
pub struct OrdFloat(f64);

impl Ord for OrdFloat {
    fn cmp(&self, OrdFloat(other): &Self) -> Ordering {
        self.0.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for OrdFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for OrdFloat {
}

// 1. Update the function signature to accept and return references to Locations
pub fn find_most_dense_location(locations: &[Location]) -> Result<&Location, Box<dyn Error>> {
    locations
        .iter()
        .max_by(|a, b| {
            a.density()
                .partial_cmp(&b.density())
                .unwrap_or(Ordering::Equal)
        })
        .ok_or("No locations found".into())
}

pub fn find_nearest_location(locations: &[Location]) -> Result<&Location, Box<dyn Error>> {
    // 2. Find the nearest location
    // Only consider locations with a density of 1000 or more
    locations
        .iter()
        .filter(|location| location.density() >= 1000.0)
        .min_by_key(|location| location.distance())
        .ok_or("No location found".into())
}

const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

#[derive(Debug)]
pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

impl Deref for SnowKg {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

impl Deref for SnowLb {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl Deref for Snowball {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SnowKg> for Snowball {
    fn from(kg: SnowKg) -> Self {
        let snowballs = (*kg / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball(snowballs)
    }
}

impl From<SnowLb> for Snowball {
    fn from(lb: SnowLb) -> Self {
        let snowballs = (*lb / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball(snowballs)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub area: f64,
    pub snow: Snowball,
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64, area: f64, snow: impl Into<Snowball>) -> Self {
        Self {
            x,
            y,
            z,
            area,
            snow: snow.into(),
        }
    }

    pub fn density(&self) -> f64 {
        if self.area > 0.0 {
            *self.snow as f64 / self.area
        } else {
            0.0
        }
    }

    pub fn distance(&self) -> OrdFloat {
        OrdFloat(self.x.hypot(self.y))
    }
}

#[cfg(test)]
mod test {
    use crate::{find_nearest_location, Location, Snowball};

    #[test]
    fn test_nearest() {
        let locations: Vec<Location> = vec![
            Location::new(10.0, 10.0, 0.0, 1.0, Snowball(1000)),
            Location::new(10.0, -10.0, 0.0, 1.0, Snowball(1000)),
            Location::new(-10.0, -10.0, 0.0, 1.0, Snowball(1000)),
            Location::new(14.0, 0.0, 0.0, 1.0, Snowball(1000)),
            Location::new(-10.0, 10.0, 0.0, 1.0, Snowball(1000)),
        ];
        assert_eq!(find_nearest_location(&locations).ok(), Some(&Location::new(14.0, 0.0, 0.0, 1.0, Snowball(1000)),));
    }

    #[test]
    fn test_nearest_density() {
        let locations: Vec<Location> = vec![
            Location::new(1.0, 0.0, 0.0, 1.0, Snowball(800)),
            Location::new(2.0, 0.0, 0.0, 1.0, Snowball(900)),
            Location::new(3.0, 0.0, 0.0, 1.0, Snowball(1000)),
            Location::new(4.0, 0.0, 0.0, 1.0, Snowball(1100)),
            Location::new(5.0, 0.0, 0.0, 1.0, Snowball(1200)),
        ];
        assert_eq!(find_nearest_location(&locations).ok(), Some(&Location::new(3.0, 0.0, 0.0, 1.0, Snowball(1000))));
    }
}
