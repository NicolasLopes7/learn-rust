use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]

struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            " x, y\n({top}, {right})\n({left}, {bottom})",
            top = self.top_left.y,
            right = self.bottom_right.x,
            left = self.top_left.x,
            bottom = self.bottom_right.y
        )
    }
}

fn main() {
    let name = String::from("Nicolas");
    let age = 17;
    let nicolas = Person { name, age };

    println!("{:?}", nicolas);

    let point = Point { x: 2.0, y: 0.4 };

    println!("coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let _pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", _pair.0, _pair.1);

    let Pair(integer, decimal) = _pair;

    println!("integer: {}\ndecimal: {}", integer, decimal);

    fn rect_area(rec: Rectangle) -> f32 {
        let Rectangle {
            bottom_right:
                Point {
                    x: right_edge,
                    y: bottom_edge,
                },
            top_left:
                Point {
                    x: left_edge,
                    y: top_edge,
                },
        } = rec;

        let b = (right_edge - left_edge).abs();
        let h = (top_edge - bottom_edge).abs();
        b * h
    }

    let area = rect_area(Rectangle {
        bottom_right: Point { x: 2.0, y: 5.0 },
        top_left: Point { x: 6.0, y: 10.0 },
    });

    println!("area: {}", area);

    fn square(_point: Point, length: f32) -> Rectangle {
        Rectangle {
            bottom_right: Point {
                y: _point.y,
                x: _point.x + length,
            },
            top_left: Point {
                x: _point.x,
                y: _point.y + length,
            },
        }
    }

    println!("{}", square(Point { x: 1.0, y: 1.0 }, 5.0));

    #[derive(Debug)]
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn match_event(evt: &WebEvent) {
        match evt {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted '{}'.", s),
            WebEvent::Click { x, y } => println!("clicked at ({}, {})", x, y),
        }
    }

    let events: [WebEvent; 5] = [
        WebEvent::KeyPress('x'),
        WebEvent::Paste("just for yest".to_owned()),
        WebEvent::Click { x: 20, y: 80 },
        WebEvent::PageLoad,
        WebEvent::PageUnload,
    ];

    for evt in events.iter() {
        match_event(evt)
    }

    #[allow(dead_code)]
    enum VeryLongAndVerboseEnumOfThings {
        Add,
        Subtract,
    }

    type Operations = VeryLongAndVerboseEnumOfThings;

    impl VeryLongAndVerboseEnumOfThings {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
}
