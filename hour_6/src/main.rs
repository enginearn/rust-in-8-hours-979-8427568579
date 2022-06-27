// A module is a namespace that contains functions, structures, traits, and impl projects.
// By default, the module modifier is private, but the public attributes can be set using the "pub" keyword.
// The module is a Rust code block, it can be used by any other Rust main program.
// The module is defined by the "mod" keyword.
// Define a module
// mod module_name {pub function() {}}
// Run the module
// module_name::function();
mod my_module_1 {
    pub fn test() {
        println!("Hello, World!");
    }
}

// Embedded Module
// Define an embedded module
// mod m1 {
//     mod m2 {
//         pud function( {
//             // do something
//         })
//     }
// }
// Run the embedded module
// m1::m2::function();
mod m1 {
    pub fn a() {
        println!("m1 module");
    }
    pub mod m2 { // embedded module
        pub fn b() { // embedded module's function
            println!("m2 module");
        }
    }
}

// Typically if you want to reference an external file, you can apply the "mod" keyword
// to load that file as a module, and apply "use" keyword to load the external function.
// The syntax to load an external file and function in the main file is:
// mod exFile; // "mod" loads external file
// use exFile::exFun; // "use" loads an external function

mod ex_file; // "mod" loads external file
use ex_file::ex_fun; // "use" loads an external function

// In Rust language, all function is private by default.
// If any function or module is private, it can be accessed only through its direct parent module or the module itself.
mod my_module_2 {
    pub fn a() { // "a" function is public
        println!("function a");
        b(); // call "b" function, correct!
    }

    fn b() { // "b" function is private
        println!("function b");
    }
}

// The super keyword is used to load the parent function in the child module, so that
// can call the private or public functions of the parent module.
// The syntax to load the parent function is:
// use::super::parent_function;
mod sup_module { // create a parent module
    fn a() -> i32 { // define a private function "a"
        100
    }
    pub mod sub_module { // create a child module
        use super::a; // load the parent function "a"
        pub fn b() { //define a public function "b"
            println!("{}", a()); // calls the parent function "a"
        }
    }
}

fn main() {
    my_module_1::test();

    m1::a();
    m1::m2::b();
    ex_fun(); // calls the external function

    my_module_2::a();
    // my_module_2::b();

    sup_module::sub_module::b();
}
