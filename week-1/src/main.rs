enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64) 
}

fn calculate_area(shape: Shape) -> f64 {
    let val = match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(l, b) => l * b,
        Shape::Square(s) => s * s
    };
    return val;
}

fn main() {
    let circle = Shape::Circle(4.8);
    let rect = Shape::Rectangle(4.2, 6.0);
    let sqr = Shape::Square(3.0);
    println!("the area is: {}", calculate_area(circle));
}
