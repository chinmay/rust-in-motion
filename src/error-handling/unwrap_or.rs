extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

fn main() {
    let first = serde_json::from_str::<Person>(
        r#"{
        "name":"Chinmay Raval"
    }"#,
    );

    // writing a matcher to extract value from result and then use it.
    /*let first_inner = match first {
        Ok(inner) => inner,
        Err(_) => Person {
            name: String::from("unknown"),
        },
    };*/

    // better solution would be, first.unwrap_or(default_value)
    let first_inner = first.unwrap_or(Person {
        name: String::from("unknown"),
    });

    // can use unwrap_or()_default() if Default trait is implemented on Person.

    println!("first.name = {:?}", first_inner.name);
}
