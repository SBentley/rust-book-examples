fn main() {
    println!("Hello, world!");
    another_function(711);
    println!("seven: {}", seven());
    println!("11 + 1 = {}", plus_one(11));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn seven() -> i32 {
    7
}

fn plus_one(val: i32) -> i32 {
    val + 1
}
