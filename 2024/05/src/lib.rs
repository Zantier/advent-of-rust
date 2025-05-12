pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

#[derive(Debug, PartialEq)]
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        // 🎁 Transform the CSV row into a Kid struct for Santa's list!
        // 🎅 Expected CSV: "Name,GoodDeeds,BadDeeds"
        //    Example: "Alice,3,1" -> name: "Alice", good_deeds: 3, bad_deeds: 1

        // 🎁 Your code here! 🎁
        let mut split = csv_row.split(",");
        let name = split.next().ok_or("name field missing")?;
        let good_deeds = split.next().ok_or("good_deeds field missing")?;
        let bad_deeds = split.next().ok_or("bad_deeds field missing")?;

        let good_deeds = good_deeds.parse().ok().ok_or("Failed to parse good_deeds")?;
        let bad_deeds = bad_deeds.parse().ok().ok_or("Failed to parse bad_deeds")?;

        Ok(Self::new(name.to_string(), good_deeds, bad_deeds))
    }

    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Self { name, niceness }
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}

#[cfg(test)]
mod test {
    use crate::{Kid, Niceness};

    #[test]
    fn test_parse_row() {
        assert_eq!(Kid::parse_row("Alice,3,1"), Ok(Kid { name: "Alice".to_string(), niceness: Niceness::Naughty }));
        assert_eq!(Kid::parse_row("Alice,6,1"), Ok(Kid { name: "Alice".to_string(), niceness: Niceness::Nice(6) }));
    }

    #[test]
    fn test_parse_row_errors() {
        assert!(Kid::parse_row("Alice").is_err());
        assert!(Kid::parse_row("Alice,3").is_err());
        assert!(Kid::parse_row("Alice,3,hi").is_err());
        assert!(Kid::parse_row("Alice,hi,1").is_err());
    }
}
