fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!("Hello {}, you will be turning to {} next", name, next_age);
}

fn square(num: i32) -> i32 {
    num * num
}

fn print_discount(day: u8) {
    let discount = if day % 2 == 0 { 50 } else { 10 };

    println!("your discount is: {}", discount);
}

fn match_number(x: &str) {
    match x {
        "1" => println!("One"),
        "2" => println!("Two"),
        "3" => println!("Three"),
        _ => println!("outside match"),
    }
}

enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hour) => println!("time is {} hour", hour),
        Clock::Digital(hour, minutes) => println!("time is {} hour and {} minutes", hour, minutes),
        Clock::Analog(hour, minutes, seconds) => println!(
            "time is {} hour {} minutes {} seconds",
            hour, minutes, seconds
        ),
        _ => println!("invalid entry"),
    }
}

struct CricketPlayer {
    name: String,
    age: u8,
    runs_ytd: u32,
}

use std::io;

fn main() {
    let a = true;
    let b = false;
    if a {
        println!("a is true");
    }
    if b {
        println!("b is true");
    }

    next_birthday("Jake", 33);
    let x = 33;
    println!("square of {} is: {}", x, square(x));

    print_discount(25);
    print_discount(26);

    for i in 1..10 {
        println!("i= {}", i);
    }

    let x = 3;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("outside match"),
    }

    /*    loop {
            println!("what is the secret word?");
            let mut word = String::new();
            io::stdin()
                .read_line(&mut word)
                .expect("Failed to read line");

            if word.trim() == "rust" {
                break;
            }
        }

    */

    /*
    let mut word = String::new();
    while word.trim() != "rust" {
        println!("what is the secret word?");
        word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read");
    }
    */

