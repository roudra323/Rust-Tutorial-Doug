enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 3.1415 * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn describe(&self) {
        match self {
            Shape::Circle { radius } => {
                println!("Circle with radius {}", radius);
            }
            Shape::Rectangle { width, height } => {
                println!("Rectangle {}x{}", width, height);
            }
            Shape::Triangle { base, height } => {
                println!("Triangle with base {} and height {}", base, height);
            }
        }
    }

    fn square(side: f64) -> Shape {
        Shape::Rectangle {
            width: (side),
            height: (side),
        }
    }
}

fn main() {
    let circle: Shape = Shape::Circle { radius: 5.0 };
    let rectangle: Shape = Shape::Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let square: Shape = Shape::square(7.0); // Using our associated function

    circle.describe();
    println!("Area: {}\n", circle.area());

    rectangle.describe();
    println!("Area: {}\n", rectangle.area());

    square.describe();
    println!("Area: {}", square.area());
}
