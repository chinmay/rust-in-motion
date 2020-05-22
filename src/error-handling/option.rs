/*

Option type -> Some or None

*/

fn main() {
    let nonempty_list = vec!['a', 'b', 'c'];
    println!("nonempty_list's last = {:?}", nonempty_list.last()); // Some('c')

    let empty_list: Vec<char> = vec![];
    println!("empty_list's last = {:?}", empty_list.last()); // None
}
