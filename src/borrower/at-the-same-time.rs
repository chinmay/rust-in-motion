//At the same time (in same lexical scope)
fn main() {
    let mut list = vec![1, 2, 3];

    let list_first = list.first();
    let list_last = list.last();

    println!(
        "First element is: {:?}, last is: {:?}",
        list_first, list_last
    );

    *list.first_mut().expect("list was empty") += 1;
    // 2 immutable & 1 mutable, violates EITHER rule, but since they are in same scope and mutable is after all immutable, it is allowed because it happens at the same time.
}
