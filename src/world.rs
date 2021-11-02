pub struct World;

impl World {
    pub fn say() {
        println!("World! {}", crate::hello::Hello::say());
    }
}
