#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// fn to calculate area of rectangle
fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 },
    } = rect;
    let width: f32 = (x1 - x2).abs();
    let height: f32 = (y1 - y2).abs();
    width * height
}

// fn to get a square Rectangle
fn square(point: Point, side: f32) -> Rectangle {
    let Point {
        x: point_x,
        y: point_y,
    } = point;
    Rectangle {
        p1: Point {
            x: point_x,
            y: point_y,
        },
        p2: Point {
            x: point_x + side,
            y: point_y + side,
        },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    /*
     * Activity
     */
    let rect = Rectangle {
        p1: Point { x: 2.4, y: 3.0 },
        p2: Point { x: 5.0, y: 6.1 },
    };
    println!("Rectangle area: {:.3}", rect_area(rect));

    println!("Square: {:?}", square(Point { x: 1.0, y: 2.0 }, 5.0));
}
