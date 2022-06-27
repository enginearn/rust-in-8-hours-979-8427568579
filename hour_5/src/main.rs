fn main() {
    let clerk = Member {
        id: 016320,
        name: "Clerk".to_string(),
        working: true,
    };

    println!("ID: {}", clerk.id);
    println!("Name: {}", clerk.name);
    println!("Working: {}", clerk.working);

    let table = Square {len: 10, wid: 10,};
    println!("Area: {}", table.len * table.wid);

    program(Language::Rust);
    program(Language::Python);
    program(Language::C);
    program(Language::Cpp);
    program(Language::Java);

    // When a binding variable owns a resource, it is called as the "ownership".
    // But when a binding variable goes out of scope, it will release the resource, and will lose ownership.
    let x = String::from("try");
    let y = x; // WANING!! The ownership of x moves to y.
    // println!("{}", x); // ERROR!! x is no longer available.

    let s = String::from("Rust in 8 Hours"); // s owns "Rust in 8 Hours"
    let n = cal(s);
    // println!("Value of the string is '{}'", s); // s is no longer available.
    println!("Length of the string is '{}'", n); // ok

    // If a variable is referenced by other variables, the ownership of its value still remains,
    // and the value of the variable will not lose.
    // Reference a variable
    // &variale_name
    // parameter: &type
    let s = String::from("Rust in 8 Hours"); // define a string s
    let n = ref_cal(&s); // reference a string s
    println!("Value of the string is '{}'", s);
    println!("from ref_cal:\nLength of the string is '{}'", n);
}

// Struct is a user-defined data type that defined by using the struct keyword.
    // The members of the struct contain member's names and types which are enclosed in braces.
    // The struct members are called fields.
    // Create a struct
    // struct Struct_name {
    //     field_name: field_type,
    //     field_name: field_type,
    //     field_name: field_type,
    // }
    // Initialize the struct
    // let object = Struct_name {
    //     field_name: field_type,
    //     field_name: field_type,
    //     field_name: field_type,
    // }
    // Access the members of the struct
    // object.field_name
struct Member {
    id: i32,
    name: String,
    working: bool,
}

struct Square {
    len: i32,
    wid: i32,

}

// Enumeration is a custom data type that contains certain values.
// Enum members own the feature of permutation and correlation.
// Define an enum
// enum Enum_Name {
//     member1,
//     member2,
// }
// Access to member
// Enum_Name::member1
enum Language {
    Rust,
    Python,
    C,
    Cpp,
    Java,
}

fn program(var: Language) {
    match var {
        Language::Rust => println!("Rust"),
        Language::Python => println!("Python"),
        Language::C => println!("C"),
        Language::Cpp => println!("C++"),
        Language::Java => println!("Java"),
    }
}

fn cal(s: String) -> usize { // define a function cal()
    s.len() // get the length of the string, but in vain
}

fn ref_cal(s: &String) -> usize { // define a function ref_cal()
    s.len() // get the length of the string, but in vain
}