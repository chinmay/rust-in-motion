fn main() {
    let s = String::from("book");

    println!("I have one {}, you have two {}", s, pluralize(&s));
}

fn pluralize(singular: &str) -> String {
    singular.to_owned() + "s"
}
