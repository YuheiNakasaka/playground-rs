pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub trait Geometry {
    fn area(&self) -> u32 {
        0
    }
}

impl Geometry for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn draw(geometry: &impl Geometry) {
    println!("Area is {}.", geometry.area());
}

// fn draw<T: Geometry>(geometry: &T) {
//     println!("Area is {}.", geometry.area());
// }

// fn draw<T>(geometry: &T)
// where
//     T: Geometry,
// {
//     println!("Area is {}.", geometry.area());
// }

// example
// let rectangle = Rectangle {
//   width: 30,
//   height: 50,
// };
// draw(&rectangle)
