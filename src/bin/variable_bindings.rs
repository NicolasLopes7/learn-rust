fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    // Variable bindings are immutable by default, but this can be overridden using the mut modifier.
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // you can create blocks in your code, blocks are basically a collection of statments enclosed by braces
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding); <- it throw an error because this variable doesn't exist in this scope

    println!("outer long: {}", long_lived_binding);

    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    // we can also declare a binding first and assigne a value from them later.
    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let mut _mutable_integer = 71i32;

    {
        // i can prevent mutability in other scope
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50; as i removed the mut keyword before, the mutable_integer is frozen
    }

    // but here the same binding works fine!
    _mutable_integer = 3;
}
