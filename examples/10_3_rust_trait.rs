trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side {}", self.side);
    }
}

fn main() {
    let circle: Circle = Circle { radius: 5.0 };
    let square: Square = Square { side: 3.0 };

    circle.draw();
    square.draw();
}
