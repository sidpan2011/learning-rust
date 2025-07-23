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

    let mut s1 = String::from("Sidhanth"); // mutable s1 so that we can pass the mut reference
    do_something(&mut s1);


    let circle = Shape::Circle(4.8);
    let rect = Shape::Rectangle(4.2, 6.0);
    let sqr = Shape::Square(3.0);
    println!("the area is: {}", calculate_area(circle));

    let str = first_find_a(String::from("sidhanth"));
    match str {
        Some(val) => println!("index is {}", val),
        None => println!("a not found")
    }
}

// first find a

fn first_find_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}




// borrowing concept

fn do_something(s2: &mut String){
    s2.push_str(" Pandey");
    println!("updated str {}", s2)
}