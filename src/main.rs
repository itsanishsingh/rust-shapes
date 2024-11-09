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

struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

impl Triangle {
    fn new(side1: f64, side2: f64, side3: f64) -> Triangle {
        Triangle {
            side1,
            side2,
            side3,
        }
    }
}

impl ShapeOperations for Triangle {
    fn area(&self) -> f64 {
        let s: f64 = self.perimeter() / 2.0;
        (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}

enum ShapeType {
    Rectangle(Rectangle),
    Square(Square),
    Circle(Circle),
    Triangle(Triangle),
}

impl ShapeType {
    fn type_name(&self) -> &str {
        match self {
            Self::Circle(_) => "Circle",
            Self::Rectangle(_) => "Rectangle",
            Self::Square(_) => "Square",
            Self::Triangle(_) => "Triangle",
        }
    }
}

impl ShapeOperations for ShapeType {
    fn area(&self) -> f64 {
        match self {
            Self::Rectangle(r) => r.area(),
            Self::Square(s) => s.area(),
            Self::Circle(c) => c.area(),
            Self::Triangle(t) => t.area(),
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Self::Rectangle(r) => r.perimeter(),
            Self::Square(s) => s.perimeter(),
            Self::Circle(c) => c.perimeter(),
            Self::Triangle(t) => t.perimeter(),
        }
    }
}

fn main() {
    let length: f64 = 6 as f64;
    let width: f64 = 8 as f64;

    let size: f64 = 5 as f64;

    let radius: f64 = 7 as f64;

    let side1: f64 = 3 as f64;
    let side2: f64 = 4 as f64;
    let side3: f64 = 5 as f64;

    let shape1 = ShapeType::Rectangle(Rectangle::new(length, width));
    let shape2 = ShapeType::Square(Square::new(size));
    let shape3 = ShapeType::Circle(Circle::new(radius));
    let shape4 = ShapeType::Triangle(Triangle::new(side1, side2, side3));

    let all_shapes = [shape1, shape2, shape3, shape4];

    for shape in all_shapes {
        println!(
            "The perimeter of shape {} is {}",
            shape.type_name(),
            shape.perimeter()
        );
        println!(
            "The area of shape {} is {}",
            shape.type_name(),
            shape.area()
        );
    }
}
