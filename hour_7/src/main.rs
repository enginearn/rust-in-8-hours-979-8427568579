fn main() {
    // A vector is actually a dynamic or mutable array.
    // It is a single data structure that can store multiple values in memory.
    // There are three ways to create a vector.

    // The first method to create a vector is:
    // let vector_name = vec![value1, value2, value3];
    let v = vec![100, 200, 300];
    println!("{}", v[0]);
    println!("{}", v[1]);
    println!("{}", v[2]);

    // The second method to create a vector is:
    // let v = vec![value; repeat];
    let v = vec![100; 3];
    println!("First element is '{}'", v[0]);
    println!("Second element is '{}'", v[1]);
    println!("Third element is '{}'", v[2]);

    // The third method to create a vector is:
    // let mut v = Vec::new(); // use "new" keyword to create a vector.
    // v.push(value1); // set value to vector.
    let mut v = Vec::new(); // create a vector.
    v.push('R'); // set value to vector.
    v.push('u'); // set value to vector.
    v.push('s'); // set value to vector.
    v.push('t'); // set value to vector.
    for c in v {
        print!("{}", c);
    }

    // You can use "|" symbol to match multiple patterns.
    let num: i32 = 5;
    match num {
        1 | 2 | 3 => println!("One, two, or three"),
        _ => println!("Something else"),
    }

    // The symbol "..." can match the values within the specific range.
    let x = 3;
    match x {
        1..=5 => println!("x is 1, 2, 3, 4, or 5"),
        _ => println!("x is something else"),
    }

    // Binding a range
    let x = 4;
    match x {
        1 => println!("one"),
        var@2..=6 => println!("var is {}", var),
        _ => println!("others"),
    }

    // Generics
    // The argument in a function can accept multiple types of data.
    // This can be done with generics. Generics are also known as parametric polymorphism.
    // THE Rust standard library provides Option for generics.
    // The syntax of the generics data type is:
    // enum Optional<T> {
    //     Some(T),
    //     None,
    // }
    let x: Option<bool> = Some(true);
    let y: Option<i32> = Some(10);
    let z: Option<f64> = Some(3.14);
    let n: Option<i32> = None;
    match x {
        Some(x) => println!("x is '{}'", x),
        None => println!("x is 'None'"),
    }
    match y {
        Some(y) => println!("y is '{}'", y),
        None => println!("y is 'None'"),
    }
    match z {
        Some(z) => println!("z is '{}'", z),
        None => println!("z is 'None'"),
    }
    match n {
        Some(n) => println!("n is '{}'", n),
        None => println!("n is 'None'"),
    }
}
