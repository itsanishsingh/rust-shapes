struct Rectangle {
    length: f64,
    width: f64,
}

trait ShapeOperations {
    fn new(length: f64, width: f64) -> Self;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

impl ShapeOperations for Rectangle {
    fn new(length: f64, width: f64) -> Rectangle {
        Rectangle { length, width }
    }

    fn perimeter(&self) -> f64 {
        return 2 as f64 * (self.length + self.width);
    }

    fn area(&self) -> f64 {
        return self.length * self.width;
    }
}

fn main() {
    let length: f64 = 5 as f64;
    let width: f64 = 3 as f64;
    let rect = Rectangle::new(length, width);
    println!("The perimeter is {}", rect.perimeter());
    println!("The area is {}", rect.area());
}
