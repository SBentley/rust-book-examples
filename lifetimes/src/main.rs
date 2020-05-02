// &i32        - A reference
// &'a i32     - A reference with an explicit lifetime
// &'a mut i32 - A mutable reference with an explicit lifetime

fn main() {
    println!("*** LIFETIMES ***");
    let string1 = String::from("abcd");
    let string2 = "xzy";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("looong striiing");

    {
        let string2 = String::from("short string");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str)  -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

