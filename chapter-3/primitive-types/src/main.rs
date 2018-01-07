use std::mem;

fn main() {
    println!("Hello, world!");
    bool_type(true);
    char_type('A');
    numeric_types();
    arrays();
    slicing();
    tuples();
}

fn bool_type(x : bool) {
    if x {
        let size_of_bool = mem::size_of::<bool>();
        println!("bool keyword lets us declare variables of boolean types. A bool type is of size {:?}", size_of_bool);
    }
}

fn char_type(x : char) {
    println!("char types can be used to represent a single character like {}. it's represented in unicode, so its size is not 1 byte, but {:?}!", x, mem::size_of::<char>());
}

fn numeric_types() {
    println!("Rust supports integer, floating-point numbers. they can be signed/unsigned. if a type starts with u, it's unsigned. otherwise, it's signed.", );
    let x = 1; // defaults to i32
    let x = 1.0; // defaults to f64
    println!("Rust supports bits in the sizes of 8, 16, 32 and 64. These are fixed-types. For the variable types, the types are called usize for unsigned and isize for signed.", );
}

fn arrays() {
    println!("Arrays are fixed-size elements beloning to the same type and immutable by default.");
    let a = [4, 5];
    println!("Immutable array - {:?}", a);
    // a[0] = 3; // Not possible.
    let mut a = [4, 5];
    a[0] = 3;
    // a[2] = 6; Not possible either. size is fixed.
    println!("After mutating array - {:?}", a);
    println!("array elements can be initialized in one go using the [T; N] notation, where T is the value and N is the number of elements.");
    let mut a = [0.7; 5];
    println!("{:?}", a);
    println!("Using an index out of bounds of the array will result in a panic!");
    // println!("{:?}", a[5]); // Runtime error.
}

fn slicing() {
    println!("Slicing is referring to a portion of an array. It's a pointer to the data and the offset.", );
    let a = [0; 5];
    let mut a = &a[..];
    println!("{:?}", a);
    println!("Slicing can be done by prefixing the variable with a & to indicate it's a reference and passing the start and end indices separated by .. Both the indices are optional, so the boundary values are assumed if left out. Moreover, the end index is exclusive, so an end index of 5 means elements till index 4 will be sliced.");
    a = &a[..5];
    println!("{:?}", a);
}

fn tuples() {
    println!("Tuples are fixed-size ordered lists of heterogeneous types. Their values are enclosed within parantheses separated by comma. This applies when specifying types on the variable.");
    let x = ("hello", "world", 34);
    println!("{:?}", x);
    println!("Tuples can be assigned to other tuples, provided the arity and types match. Arity is nothing but the size.");
    let y = x;
    println!("{:?}", y);
    // let y : (&str, &str, &str) = x; // This is an error.
    println!("Tuple elements can be accessed using a destructuring let. The destructuring let should reference all elements inside a tuple, not a few.", );
    let (a, b, c) = x;
    println!("{:?} {} {}", a, b, c);
    println!("Single element tuples can be disambiguated from a value by adding a comma after that single element.");
    let a = (5, );
    println!("{:?}", a);
    let a = 5;
    println!("{:?}", a);
    println!("Tuples can be accessed through indices like arrays, but unlike arrays, indices are specified after a dot, not inside square brackets.");
    let a = ("hello", 1.0, 54);
    println!("{:?} {:?} {:?}", a.0, a.1, a.2);
}
