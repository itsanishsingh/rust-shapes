struct Rectangle {
    length: f64,
    width: f64,
}

struct Square {
    size: f64,
}

// trait ShapeOperations {
//     fn new() -> Self;
//     fn area(&self) -> f64;
//     fn perimeter(&self) -> f64;
// }

impl Rectangle {
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

impl Square {
    fn new(size: f64) -> Square {
        Square {size}
    }

    fn area(&self) -> f64 {
        return self.size*self.size;
    }

    fn perimeter(&self) -> f64 {
        return self.size*4 as f64;
    }
}

fn main() {
    let length: f64 = 5 as f64;
    let width: f64 = 3 as f64;

    let size: f64 = 4 as f64;

    let rect = Rectangle::new(length, width);
    let sq = Square::new(size);

    println!("The perimeter of rectangle is {}", rect.perimeter());
    println!("The area of rectangle is {}", rect.area());

    println!("The perimeter of square is {}", sq.perimeter());
    println!("The area of square is {}", sq.area());
}
