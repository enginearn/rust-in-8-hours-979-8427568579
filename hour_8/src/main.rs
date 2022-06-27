// A function in struct, enum, trait is called method.
// The syntax to define a method is:
// impl Struct/Enum {
//     fn method_name(&self) -> type_of_return_value {
//         // code
//         self.member_namer // access the member variable.
//     }
// }
/*
The first letter of the struct name should be capitalized.
The parameter "&self" indicates that the method is using a referenced parameter.
Usually, if we use a referenced parameter in a method, then the method doesn't need to
use "return" statement. The referenced parameter can return the value to the method caller.
*/

struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

/*
Trait is an interface in Rust, it defines a trait method without
the body and is implemented by an "impl" method.
*/
// Define a trait and its method.
// trait Trait_Name {
//     fn trait_method(&self);
// }
// The first letter of the trait name should be capitalized.
// The trait method has no body.

// Implement the trait method.
// impl Trait_Name for Struct/Enum {
//     fn trait_method(&self) {
//         // code
//         self.member_namer // access the member variable.
//     }
// }

trait TraitCalculateArea {
    fn trait_area(&self) -> f64;
}

impl TraitCalculateArea for Circle {
    fn trait_area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

pub trait Show {
    fn show(&self);
}

impl<T> Show for T
where
    T: ToString,
{
    fn show(&self) {
        println!("{}", self.to_string());
    }
}

/*
The drop() method will be automatically invoked when the last "_variable"
expression finishes running.
The feature of drop() method: "Last in, First out".
*/
// The syntax to define a drop() method is:
// fn drop(&mut self) {
//     // execute commands according to "Lat in , First out"
// }

// The syntax to define a "_variable" expression is:
// let _variable = Strict {member: value}

struct Game {
    number: i32,
}

impl Drop for Game {
    fn drop(&mut self) {
        println!("The #{} Winner.", self.number);
    } // drop method runs the code based on "Last in First out".
}

// The clousure is the anonymous function of Rust Language.
// Create a closure or an anonymous function.
// let closure_name = |parameter| {};

// call the closure or anonymous function.
// closure_name(parameter);

fn main() {
    let circle = Circle { radius: 200.0 };
    println!("The area of the circle is {}", circle.area());

    let trait_area = Circle { radius: 150.00 };
    println!("The Circle area is '{}'", trait_area.trait_area());

    String::from("Rust in 80Hours").show();

    let _baseball = Game { number: 3 }; // define _baseball
                                        // _baseball expression runs last
    let _football = Game { number: 2 };
    // _football expression runs next
    let _basketball = Game { number: 1 };
    // _basketball expression runs first

    let my_closure = |num: i32| num + 200;
    let num = 100;
    println!("{}", my_closure(num));

    let mut capacity = "Hard disk capacity: 5000".to_string();
    {
        let mut my_closure = |c: char| capacity.push(c);
        my_closure('G');
    }
    println!("{}", capacity);
}
