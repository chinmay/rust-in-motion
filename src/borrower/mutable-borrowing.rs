//Mutable borrowing

#[derive(Debug)]
struct CarPool {
    pessangers: Vec<String>, // list of pessangers
}

impl CarPool {
    //&mut self => Let me make changes to values of struct CarPool
    fn add_pickup(&mut self, name: String) {
        self.pessangers.push(name);
    }
}

fn main() {
    let mut monday_car_pool = CarPool { pessangers: vec![] }; //this let needs to be mutable as well.

    monday_car_pool.add_pickup(String::from("Naresh"));
    println!("CarPool status: {:?}", monday_car_pool);

    monday_car_pool.add_pickup(String::from("Mukti"));
    println!("CarPool status: {:?}", monday_car_pool)
}
