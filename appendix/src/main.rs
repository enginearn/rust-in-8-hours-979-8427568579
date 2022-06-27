/*
"assert!" is used to check errors. Suppose you declare a variable to be true first,
and if it is false after comparison, the program stops executing.
*/
fn main() {
    let check: bool = true;
    assert!(check == true);
    print!("{}", check);
}
