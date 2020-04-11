fn main() {
    // string literals are immutable, use String::from for mutability
    let s1 = String::from("hello");
    let s2 = s1;
    println!("String 2 len: {}", s2.len());
    clone_string(s2);
    stack_copy();
    string_slice(s2);
}

// String is allocated on the heap so we have to clone
// We are not allowed to have two pointers pointed at the same value
fn clone_string(s: String) {
    let s2 = s.clone();
    println!("s = {}, s2 = {}", s, s2);
}

// We can copy variables that live on the stack.
fn stack_copy() {
    let x = 11;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn string_slice(s :String) {
    println!("{}", &s[0..5]);
}
