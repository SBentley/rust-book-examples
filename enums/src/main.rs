enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move{ x: u32, y: u32},
    Write(String),
    ChangeColour(i32, i32, i32),
}
#[derive(Debug)]
enum OlympicSport {
    Archery,
    Handball,
    Javelin,
    Track,
}

enum Coin {
   one_p,
   two_p,
   five_p,
   ten_p,
   fifty_p(OlympicSport),
}

// Implement a method on an enum
impl Message {
    fn call(&self) {
        // method body
        println!("Message method call has been invoked");
    }
}

fn main() {
    println!("***Enums***");
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));
    route(four);

    let m = Message::Write(String::from("Write Message"));
    m.call();
    println!("Calling value in pence with a 1p");
    println!("Value of Coin::one_p is {}", value_in_pence(Coin::one_p));
    println!("Value of (Handball) coin is {}", value_in_pence(Coin::fifty_p(OlympicSport::Handball)));
}

fn route(ip_kind: IpAddrKind) {
    println!("Function not implemented");
}

fn value_in_pence(coin: Coin) -> u8 {
    match coin {
        Coin::one_p => {
            println!("Ooh a penny!");
            1
        },
        Coin::two_p => 2,
        Coin::five_p => 5,
        Coin::ten_p => 10,
        Coin::fifty_p(sport) => {
            println!("Olympic 50p with sport: {:?}",sport);
            50
        },
    }
}
