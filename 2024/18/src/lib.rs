#[derive(Clone)]
pub struct Sleigh {
    color: String,
    engine: String,
    gift_capacity: u32,
    magical_enhancements: bool,
}

#[derive(Clone)]
pub struct SleighBuilder {
    // Define the fields of SleighBuilder here
    sleigh: Sleigh
}

impl SleighBuilder {
    // Your code here...
    pub fn new() -> Self {
        Self {
            sleigh: Sleigh {
                color: "red".to_string(),
                engine: "reindeer-powered".to_string(),
                gift_capacity: 100,
                magical_enhancements: false
            }
        }
    }

    pub fn color(mut self, color: &str) -> Self {
        self.sleigh.color = color.to_string();
        self
    }

    pub fn engine(mut self, engine: &str) -> Self {
        self.sleigh.engine = engine.to_string();
        self
    }

    pub fn gift_capacity(mut self, gift_capacity: u32) -> Self {
        self.sleigh.gift_capacity = gift_capacity;
        self
    }

    pub fn magical_enhancements(mut self) -> Self {
        self.sleigh.magical_enhancements = true;
        self
    }

    pub fn build(self) -> Sleigh {
        self.sleigh
    }
}

impl Default for SleighBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Don't Change this implementation
// It is used for the tests
impl Sleigh {
    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn engine(&self) -> &str {
        &self.engine
    }

    pub fn gift_capacity(&self) -> u32 {
        self.gift_capacity
    }

    pub fn magical_enhancements(&self) -> bool {
        self.magical_enhancements
    }
}

pub fn main() {
    let sleigh = SleighBuilder::new()
        .color("gold")
        .engine("magic")
        .gift_capacity(350)
        .magical_enhancements()
        .build();

    assert_eq!(sleigh.color(), "gold");
    assert_eq!(sleigh.engine(), "magic");
    assert_eq!(sleigh.gift_capacity(), 350);
    assert!(sleigh.magical_enhancements());
}
