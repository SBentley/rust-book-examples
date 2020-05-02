Use std::fmt::Display;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if (item > largest) {
            largest = item;
        }
    }
    

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {    
    fn x(&self) -> &T {
        &self.x
    }
}

struct DifferingPoints<T, U> {
    x: T,
    y: U,
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x: {}", self.x);
        }
        else  {
            println!("The largest member is y: {}", self.y);
        }
    }
}

fn main() {
    println!("*** Generics! ***");
    let numbers = vec![1, 24, 75, 8, 11, 55];
    let result = largest(&numbers);
    println!("The largest number in the list is {}", result);

    let chars = vec!['r', 'd', 's', 'i', 'l'];
    let result = largest(&chars);
    println!("The largest char in the list is {}", result);    

    let int = Point { x: 5, y: 10 };
    let float = Point { x: 7.2, y: 20.0 };
    let different = DifferingPoints { x: 5, y: 20.0 };
}
