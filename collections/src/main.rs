mod vectors {
    pub fn create_vectors() {
        let _v: Vec<i32> = Vec::new();
        // Infer the type from the initial values
        let _v1 =  vec![1,2,3];
        
        // Type is also infered here as values in push are all i32
        let mut v2 = Vec::new();
        v2.push(5);
        v2.push(6);
        v2.push(7);
        v2.push(8);        

    } // vectors go out of scope and are freed. elements are also freed here

    pub fn read() {
        let v = vec![1,2,3,4,5];

        let third: &i32 = &v[2];
        println!("The thid element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}.", third),
            None => println!("There is no third element."),
        }
        
       // let does_not_exist = &v[100]; // This will cause a panic
        let does_not_exist = v.get(100); // This will return None

        for i in &v {
            println!("{}", i);
        }

        let mut v = vec![100,27,11,89];
        for i in &mut v {
            *i += 50; // use dereference operator to get value of mutable reference
        }        
    }

    fn multiple_types() {
        let row = vec![
            SpreadsheetCell::Int(7),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(11.79)
        ];
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
}

mod Strings {
    pub fn basics() {
        let mut s = String::new();

        let data = "initial contents";
        let s = data.to_string();
        // we can call to_string directly on a literal
        let s = "initial contents".to_string();
        let mut s = String::from("initial contents");

        s.push_str(" second contents");
        s.push('.');
        println!("{}", s);
    }

    pub fn concat() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world.");
        let s3 = s1 + &s2; // s1 has been moved here and no longer usable

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);

        let len = String::from("Здравствуйте").len(); // 24 not 12 because of utf-8 needing 2 bytes
        
        let hello = "Здравствуйте";
        let s = &hello[0..4]; // s = Зд as each character is 2 bytes. Latin chars are 1 byte
        println!("{}",s);
    }
}

mod HashMaps {
use std::collections::HashMap;
    pub fn create() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10,50];
        let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

        let team_name = String::from("Blue");
        let score = scores.get(&team_name); // Retuns an Option<&V> as no guarantee value exists
        
        for (key, value) in &scores {
            println!("{}: {}", key, value)
        }
    }
}

fn main() {
    println!("Hello, world!");
    vectors::create_vectors();
    vectors::read();
    Strings::basics();
    Strings::concat();
    HashMaps::create();
}


