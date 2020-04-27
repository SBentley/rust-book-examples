use std::fs::File;
use std::io;
use std::fs;
use std::io::Read;
use std::error::Error;

fn panics() {
    println!("Error Handling!");
    // panic!("Crash and burn");
    let v = vec![1,2,3];
    v[99];
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt");

    /*
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
    */
    // return value of file or panics if it doesn't exist
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open file hello.txt");

    let f = File::open("hello.txt")?;
    Ok(())
}

// Propagate any error
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Use the shortcut ? for error propagation
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Chain together ? expressions
fn read_username_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

}


fn read_username_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
