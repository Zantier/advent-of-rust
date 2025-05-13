use std::marker::PhantomData;

// 1. We have 3 states:
// - Empty
// - Ready
// - Flying
pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Seligh struct definition
pub struct Sleigh<T> {
    phantom: PhantomData<T>,
}

// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
    pub fn new() -> Sleigh<Empty> {
        Sleigh { phantom: PhantomData }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { phantom: PhantomData }
    }
}

impl Default for Sleigh<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { phantom: PhantomData }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { phantom: PhantomData }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { phantom: PhantomData }
    }
}
