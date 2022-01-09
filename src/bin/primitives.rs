use std::fmt::{self, Display, Formatter};
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    let tuple = (1u8, 2u8, 3u8, 4u8, -1i8, 0.1f32, -0.1f32, 'a', true);

    println!("first element {}", tuple.0);
    println!("second element {}", tuple.1);

    let tuple_of_tuples = ((1u32, 2u32, 3u32), ('a', 'b', 'c'), 1u32);

    println!("tuple of tuples {:?}", tuple_of_tuples);
    println!("tuple of tuples {:?}", tuple);

    fn reverse_pair(pair: (i32, bool)) -> (bool, i32) {
        let (number, boolean) = pair;

        (boolean, number)
    }

    let reversed_pair = reverse_pair((10, true));
    println!("{:?}", reversed_pair);
    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "({0} {1})", (self.0 + 1f32), (self.1 + 1f32))
        }
    }

    #[derive(Debug)]
    struct Matrix(f32, f32);

    let ex_matrix = Matrix(1.1, 1.2);
    println!("{:?}", ex_matrix);
    println!("{}", ex_matrix);
    fn reverse_matrix(mtx: Matrix) -> Matrix {
        Matrix(mtx.1, mtx.0)
    }

    println!("Matrix:\n{}", ex_matrix);
    println!("Transpose:\n{}", reverse_matrix(ex_matrix));

    // Array
    let integer_array: [i32; 5] = [1, 2, 3, 4, 5];

    let empty_array: [i32; 500] = [0; 500];
    println!("{:?}", integer_array);
    println!("{} length of array", empty_array.len());

    println!("array occupies {} bytes", mem::size_of_val(&empty_array));

    analyze_slice(&integer_array);

    println!("borrow a section of the array as a slice");
    analyze_slice(&empty_array[1..4]);
}
