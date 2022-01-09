use std::fmt;

fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Nicolas", "Sérgio");

    println!(
        "{subject} {verb} {object}",
        object = "frog",
        subject = "handsome",
        verb = "jumps over"
    );
    // :b means binary
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // Right align with 5 whitespaces and the char
    println!("{number:>width$}", number = 1, width = 6);
    // pad 5 zeroes
    println!("{number:0>width$}", number = 1, width = 6);

    let pi = 3.141592;
    println!("pi is roughly {:.3}", pi);

    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    // struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Structure(i32);
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
    // There's a pretty print using "#""
    println!("{:#?}", Deep(Structure(7)));
    // fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the output appearance.
    // This is done by manually implementing fmt::Display, which uses the {} print marker.
    // Implementing it looks like this:
    impl fmt::Display for Structure {
        // this trait requires fmt with this exact signature
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }
    let ex_structure = Structure(10);
    println!("Display: {}", ex_structure);
    println!("Debug: {:?}", ex_structure);

    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use self.[index] to refer to each positional data point, if u're using a struct, u can use the key to get the value.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    let ex_min_max = MinMax(20, 20);
    println!("Display: {}", ex_min_max);
    println!("Debug: {:?}", ex_min_max);

    #[derive(Debug)]
    struct CartesianCoordinates {
        x: f64,
        y: f64,
    }

    impl fmt::Display for CartesianCoordinates {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {0}, y: {1}", self.x, self.y)
        }
    }

    let ex_cartesian_coords = CartesianCoordinates { x: 3.3, y: 7.2 };
    println!("Display: {}", ex_cartesian_coords);
    println!("Debug: {:?}", ex_cartesian_coords);

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ",")?;
                }
                write!(f, "\n{}: {}", count, v)?;
            }

            write!(f, "\n]")
        }
    }

    println!("{}", List(vec![1, 2, 3]));

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "({red}, {green}, {blue}) 0x{red:X}{green:X}{blue:X}",
                red = self.red,
                green = self.green,
                blue = self.blue
            )
        }
    }

    let white = Color {
        red: 255,
        green: 255,
        blue: 255,
    };

    println!("{}", white)
}
