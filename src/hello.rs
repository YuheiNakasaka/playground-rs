use rand::prelude::*;

pub struct Hello;

impl Hello {
    pub fn say() -> i32 {
        random()
    }
}
