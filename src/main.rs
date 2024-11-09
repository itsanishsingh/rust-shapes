trait ShapeOperations {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    length: f64,
    width: f64,
}

impl Rectangle {
    fn new(length: f64, width: f64) -> Rectangle {
        Rectangle { length, width }
    }
}

impl ShapeOperations for Rectangle {
    fn perimeter(&self) -> f64 {
        return 2 as f64 * (self.length + self.width);
    }

    fn area(&self) -> f64 {
        return self.length * self.width;
    }
}

struct Square {
    size: f64,
}

impl Square {
    fn new(size: f64) -> Square {
        Square { size }
    }
}

impl ShapeOperations for Square {
    fn area(&self) -> f64 {
        return self.size * self.size;
    }

    fn perimeter(&self) -> f64 {
        return self.size * 4 as f64;
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl ShapeOperations for Circle {
    fn area(&self) -> f64 {
        return 3.14 * self.radius * self.radius;
    }

    fn perimeter(&self) -> f64 {
        return 2 as f64 * 3.14 * self.radius;
    }
}

enum ShapeType {
    Rectangle(Rectangle),
    Square(Square),
    Circle(Circle),
}

impl ShapeOperations for ShapeType {
    fn area(&self) -> f64 {
        match self {
            ShapeType::Rectangle(r) => r.area(),
            ShapeType::Square(s) => s.area(),
            ShapeType::Circle(c) => c.area(),
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            ShapeType::Rectangle(r) => r.perimeter(),
            ShapeType::Square(s) => s.perimeter(),
            ShapeType::Circle(c) => c.perimeter(),
        }
    }
}

fn main() {
    let length: f64 = 6 as f64;
    let width: f64 = 8 as f64;

    let size: f64 = 5 as f64;

    let radius: f64 = 7 as f64;

    let shape1 = ShapeType::Rectangle(Rectangle::new(length, width));
    let shape2 = ShapeType::Square(Square::new(size));
    let shape3 = ShapeType::Circle(Circle::new(radius));

    println!("The perimeter of rectangle is {}", shape1.perimeter());
    println!("The area of rectangle is {}", shape1.area());

    println!("The perimeter of square is {}", shape2.perimeter());
    println!("The area of square is {}", shape2.area());

    println!("The circumference of circle is {}", shape3.perimeter());
    println!("The area of circle is {}", shape3.area());
}
