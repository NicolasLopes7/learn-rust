# Notes

> How to compile an binary file

`rustc [file_path] [--out-dir="/output/dir"]?`

## Concepts
<details>
<summary>Macros</summary>
Rust provides a powerful macro system that allows metaprogramming. As you've seen in previous chapters, macros look like functions, except that their name ends with a bang !, but instead of generating a function call, macros are expanded into source code that gets compiled with the rest of the program. However, unlike macros in C and other languages, Rust macros are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs.

> So why are macros useful?

- Don't repeat yourself. There are many cases where you may need similar functionality in multiple places but with different types. Often, writing a macro is a useful way to avoid repeating code. (More on this later)

- Domain-specific languages. Macros allow you to define special syntax for a specific purpose. (More on this later)

- Variadic interfaces. Sometimes you want to define an interface that takes a variable number of arguments. An example is println! which could take any number of arguments, depending on the format string!. (More on this later)

Example:

```rust
// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}
```
</details>

<details>
<summary>Primitives</summary>

### Scalar Types

- signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
- unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
- floating point: f32, f64
- char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
- bool either true or false
- and the unit type (), whose only possible value is an empty tuple: ()

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

### Compound Types

- arrays like `[1, 2, 3]`
- tuples like `(1, true)`

rust can infer types, like:
```rust
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;
}
```

tuples are destructurable


An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created using brackets `[]`, and their length, which is known at compile time, is part of their type signature `[T; length]`. For example:
```rust
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];
```

</details>
