fn main() {
    // let variable_name = value;

    let int_8: i8 = 8;
    let int_16: i16 = 16;
    let int_32: i32 = 32;
    let int_64: i64 = 64;
    let int_min_size: isize = isize::MIN; // -2 ** 63
    let int_max_isize: isize = isize::MAX; // 2 ** 63 - 1

    let un_int_8: u8 = 8;
    let un_int_16: u16 = 16;
    let un_int_32: u32 = 32;
    let un_int_64: u64 = 64;
    let un_int_min_size: usize = usize::MIN; // 0
    let un_int_max_usize: usize = usize::MAX; // 2 ** 64 - 1


    let fp32: f32 = 0.3;
    let fp64: f64 = 1.4;

    let boolean = true;
    let string = "Hello, world!".to_string();
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..];
    let tpl = (1, 2, 3, 4, 5);
    let one_word = 'r';

    println!("int_8: {}", int_8);
    println!("int_16: {}", int_16);
    println!("int_32: {}", int_32);
    println!("int_64: {}", int_64);
    println!("int_min_size: {}", int_min_size);
    println!("int_max_size: {}", int_max_isize);

    println!("un_int_8: {}", un_int_8);
    println!("un_int_16: {}", un_int_16);
    println!("un_int_32: {}", un_int_32);
    println!("un_int_64: {}", un_int_64);
    println!("un_int_min_size: {}", un_int_min_size);
    println!("un_int_max_size: {}", un_int_max_usize);

    println!("fp32: {}", fp32);
    println!("fp64: {}", fp64);

    println!("boolean: {}", boolean);
    println!("string: {}", string);
    println!("a: {:?}", a);
    println!("slice: {:?}", slice);
    println!("tpl: {:?}", tpl);
    println!("one_word: {}", one_word);

    let var: i32 = 100;
    let str: String = "Good".to_string();

    println!("var: {}", var);
    println!("str: {}", str);

    let x = 100;
    let y = 200;
    let z = 300;

    println!("x: {}", x);
    print!("y: {}, z: {}", y, z);

    // cost identifier: type = value;
    const NUM: i32 = 100;

    println!("The value of NUM: {}", NUM);

    // Data types conversion: as new_type
    let var1: f32 = 1_000.88;
    let var2: i32 = var1 as i32;

    println!("var1: {}", var1);
    println!("var2: {}", var2);

    // Function: fn function_name(arguments) -> return_type {}
    // Call function: function_name(arguments)

    funt(100, 200);

    let num = return_val_funt(100);

    println!("The return value of num: {}", num);

    let tof = true_or_false();
    println!("The return value of tof: {}", tof);

}

fn funt(x: i32, y: i32) {
    println!("sum from fn funt: {}", x + y);
}

fn return_val_funt(num: i32) -> i32 {
    num + 200
}

fn true_or_false() -> bool {
    return true
}