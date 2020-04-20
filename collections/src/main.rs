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

fn main() {
    println!("Hello, world!");
    vectors::create_vectors();
    vectors::read();
}


