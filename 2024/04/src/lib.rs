pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    // Create the enum variants `Nice` and `Naughty`
    // Variant `Nice` is a tuple struct that holds the number of good deeds
    Nice(u32),
    Naughty,
}

impl Niceness {
    fn from_deeds(good_deeds: u32, bad_deeds: u32) -> Self {
        if good_deeds == 0 && bad_deeds == 0 {
            return Self::Naughty;
        }

        let good = good_deeds as f32 * GOOD_WEIGHT;
        let bad = bad_deeds as f32 * BAD_WEIGHT;
        let ratio = good / (good + bad);

        if ratio >= 0.75 {
            Self::Nice(good_deeds)
        } else {
            Self::Naughty
        }
    }
}

pub struct Kid {
    // Add a field `name` of type `String`
    // and `niceness` field of type `Niceness`
    // Make all fields public
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        // Return a Kid instance
        Kid {
            name,
            niceness: Niceness::from_deeds(good_deeds, bad_deeds),
        }
    }

    // Move yesterday's function to an associated function in the struct
    pub fn is_nice(&self) -> bool {
        if let Niceness::Nice(_) = self.niceness {
            true
        } else {
            false
        }
    }
}
