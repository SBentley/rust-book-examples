// &i32        - A reference
// &'a i32     - A reference with an explicit lifetime
// &'a mut i32 - A mutable reference with an explicit lifetime
use std::fmt::Display;

fn main() {
    println!("*** LIFETIMES ***");
    let string1 = String::from("abcd");
    let string2 = "xzy";    

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    let ann = "My announcement";
    longest_with_an_announcement(string1.as_str(), string2, ann);

    let string1 = String::from("looong striiing");

    {
        let string2 = String::from("short string");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    struct_lifetimes();
}

fn struct_lifetimes() {
    let novel = String::from("A long time ago in a galaxy far, far away....");
    let first_caluse = novel.split(',').next().expect("Could not find a ',' in string");
    let i = ImportantExcerpt {
       part: first_caluse,
    };
    println!("{}", first_caluse);
    let announcement = String::from("This is an announcement");
    i.announce_and_return_part(announcement.as_str());
    let s: &'static str = "I have a static lifetime";
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        7
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please {}", announcement);
        self.part
    }
   
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest<'a>(x: &'a str, y: &'a str)  -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


