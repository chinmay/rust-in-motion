/*

Result type -> Ok or Err
Option type -> Some or None

Success - use value
Failure - recover

Ok(value) or Some(value) != value // types doesn't match

*/

/*

Result type -> OK or Err
Option type -> Some or None

Success - use value
Failure - recover

OK(value) or Some(value) != value // types doesn't match


*/

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

    println!("first = {:?}", first); // Ok(Person { name: "Chinmay Raval" })

    let second = serde_json::from_str::<Person>(
        r#"{
        "name":"Urmi Parikh",
    }"#,
    );
    println!("second = {:?}", second); //Err(Error("trailing comma", line: 3, column: 5))

    // Extracting values
    // since first = OK(value) != value.

    // writing a matcher to extract value from result and then use it.
    let first_inner = match first {
        Ok(inner) => inner,
        _ => unimplemented!(),
    };

    println!("first.name = {:?}", first_inner.name);

    let second_inner = match second {
        Ok(inner) => inner,
        Err(_) => Person {
            name: String::from("unknown"),
        },
    };

    println!("second.name = {:?}", second_inner.name);
}
