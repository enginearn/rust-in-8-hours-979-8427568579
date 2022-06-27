fn main() {
    // "if statemnent" executes codes inside {...} only if a specified condition is true.
    // if condition { // if true run this code } else { // if false run this code }
    let num = 10;
    if num == 10 {
        println!("num is equal to 10");
    }

    let x = 100;
    let y = 200;

    if x > y {
        println!("x is greater than y");
    } else {
        println!("x is not greater than y");
    }

    // let-if statement will assign one of the values to a variable
    // according to the condition results.
    // let valiable_name = if condition { value1 } else { value2 };

    let num = if true {
        100
    } else {
        200
    };
    println!("The value of num is {}", num);

    // loop-break statement
    // loop {... break;}
    let mut count = 0;
    loop {
        if count <= 1 {
            println!("Rust in {}hour", count);
        } else {
            println!("Rust in {}hours", count);
        }
        if count == 8 {
            break;
        }
        count += 1;
    }

    // for-statement
    // A for loop is an conditional loop that executes a block of code for each item in a sequence.
    // for var in collection {...}
    for num in 2..11 {
        println!("Python in {}hours", num);
    }

    // while-statement
    // A while loop is a conditional loop that executes a block of code while a condition is true.
    // while condition {...}
    let mut num = 2;
    while num <= 8 {
        println!("TypeScript in {}hours", num);
        num += 1;
    }

    // tuples
    // A tuple is a collection of different type values separated by commas with parentheses.
    // let tuple = (1, "Hello", true);
    let t = ("Python" , "in", 8, "hours", true);
    println!("{} {} {} {}", t.0, t.1, t.2, t.3);

    // match-statement
    // A match statement is a way to execute different code depending on the value of an expression.
    // match expression {
    //     value1 => code1,
    //     value2 => code2,
    //     ...
    //     _default => code,
    // }

    let num: i32 = 10;
    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        6 => println!("six"),
        7 => println!("seven"),
        8 => println!("eight"),
        9 => println!("nine"),
        10 => println!("ten"),
        _ => println!("not a number or out of range..."),
    }
}
