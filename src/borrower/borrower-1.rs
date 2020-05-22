struct Person {
    name: String,
}

//This function doesnt want to own the Person, so it just borrows it using &
fn congratulate(person: &Person) {
    println!("Congratulations, {}!!", person.name);
}

fn main() {
    let p = Person {
        name: String::from("Naresh"),
    };

    congratulate(&p); // passing reference.
    println!("can still use p here: {}", p.name)
}
