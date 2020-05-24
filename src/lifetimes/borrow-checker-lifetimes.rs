/* concrete lifetimes
In Rust, a value's lifetime is the time during which the value exists at a particular memory address.
A reference's lifetime must be contained within the lifetime of the value being referenced


Borrow checker errors
*/
fn main() {
    /*let first_two = {
        let list = vec![100, 34, 72, 55];
        &list[0..2]
    };*/

    let list = vec![100, 34, 72, 55];
    let first_two = &list[0..2];
    println!("first two are {:?}", first_two);

    /*first_two = return_first_two();
    println!("first two are {:?}", first_two);
    */

    let list_two = return_list();
    let first_two_2 = &list_two[0..2];
    println!("first two are {:?}", first_two_2);

    /*
    let list_a = vec![100, 34, 72, 55];
    let list_b = list_a; // lifetime of a ends here.
    let first_two_3 = &list_a[0..2];
    println!("first two are {:?}", first_two_3);
    */

    let list_a = vec![100, 34, 72, 55];
    let first_two_3 = &list_a[0..2];
    println!("first two are {:?}", first_two_3);
    let list_b = list_a;
}

/*fn return_first_two() -> &[i32] {
    let list = vec![100, 34, 72, 55];
    &list[0..2]
}*/

fn return_list() -> Vec<i32> {
    vec![100, 34, 72, 55]
}

// self referencing struct
struct ListAndRef {
    list: Vec<i32>,
    first_two: &[i32],
}

fn return_list_and_first_two() -> ListAndRef {
    let list_to_use = vec![100, 34, 72, 55];

    ListAndRef {
        list: list_to_use,
        first_two: &list_to_use[0..2],
    }
}
