fn panics() {
    println!("Error Handling!");
    // panic!("Crash and burn");
    let v = vec![1,2,3];
    v[99];
}

use std::fs::File;
fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", error),
            },
    };



}
