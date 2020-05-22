//At the same time (in same lexical scope)
fn main() {
    let mut list = vec![1, 2, 3];

    //introduce new scope to tell compiler that immutable scope ends before main, so can have new mutable borring again.
    {
        let list_first = list.first();
        let list_last = list.last();

        println!(
            "First element is: {:?}, last is: {:?}",
            list_first, list_last
        );
    }

    // new (mutable) borrowing outside scope.
    *list.first_mut().expect("list was empty") += 1;
}
