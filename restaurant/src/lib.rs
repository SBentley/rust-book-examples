mod front_of_house;

pub use crate::front_of_house::hosting;

fn serve_order() {}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Absolute Path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    // front_of_house::hosting::add_to_waitlist();

    // Order a summer breakfast on Sourdough toast
    let mut meal = back_of_house::Breakfast::summer("Sourdough");
    // Change the type of toast
    meal.toast = String::from("Rye");
    println!("I'd like {} toast please", meal.toast);

    // This is not allowed due to being a private path
    // meal.seasonal_fruit = String::from("blueberries");
    
    let order1 = back_of_house::Starter::Soup;
    let order2 = back_of_house::Starter::Salad;
    
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Starter {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
             Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Strawberries"),
             }
        }
    }
    
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order(){}
}



