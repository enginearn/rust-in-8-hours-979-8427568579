// Valiable bindings
// let (va1, var2) = (value1, value2);

fn main() {
    let (x, y) = (100, 200);
    println!("x: {}", x);
    println!("y: {}", y);

// let mut valiable = value;

let mut a = 100;
let mut b = 200;

a += 300;
b += 400;

println!("a: {}", a);
println!("b: {}", b);

// String Assignment
// let x = "text".to_string();
// let y = String::from("text");
// let z: &str = "text";

let x = "hello".to_string();
let y = String::from("hello");
let z: &str = "hello";

println!("{}, {}, {}", x, y, z);

println!("10 + 2 = {}", 10 + 2);
println!("10 - 2 = {}", 10 - 2);
println!("10 * 2 = {}", 10 * 2);
println!("10 / 2 = {}", 10 / 2);
println!("10 % 2 = {}", 10 % 2);

println!("true AND true is {}", true && true);  // returns true
println!("true OR true is {}", true || true); // returns true
println!("true AND false is {}", true && false); // returns false
println!("true OR false is {}", true || false); // returns true
println!("NOT true is {}", !true); // returns false
println!("NOT false is {}", !false); // returns true
println!("false AND false {}", false && false); // returns false
println!("false OR false {}", false || false); // returns false

// After using comparison operators, the result is always a boolean value. (true or false)
let x: i32 = 100;
let y: i32 = 200;

println!("x greater than y is {}", x > y); // returns false
println!("x is less than y is {}", x < y); // returns true
println!("x is unequal to y is {}", x != y); // returns true
println!("x is greater / equal to y is {}", x >= y); // returns false
println!("x is less / equal to y is {}", x <= y); // returns true
println!("x == y is {}", x == y); // returns false

// Arrays
// The forst way to create an array is to use the array literal syntax:
// let mut arr: [type; length] = [default; length];

let mut a:[i32; 4] = [8; 4];
println!("default: {}, {}, {}, {}", a[0], a[1], a[2], a[3]);
a[1] = 10;
a[2] = 20;
println!("assigned values: {}, {}, {}, {}", a[0], a[1], a[2], a[3]);

// The second way to create an array is to use the array type:
// let mut aaray:[type; length] = [val1, val2, val3, val4...];
let a: [f32; 4] = [0.1, 0.2, 0.3, 0.4];
println!("{}, {}, {}, {}", a[0], a[1], a[2], a[3]);

// The slice si a part of an array. To create a slice, we need to extract a portion of an array.
// The syntax to create a slice is as follows:
// let slice = &array[start..last-1];
let a = [1, 2, 3, 4, 5 ,6, 7, 8, 9, 10];
let slice = &a[2..5]; // to crate a slice with three elements.
println!("{:?}", slice);
}