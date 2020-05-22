pub struct Monster {
    hp: u8,
    sp: u8,
    friends: Vec<Friend>,
}

pub struct Friend {
    loyalty: u8,
}

impl Monster {
    pub fn final_breath(&mut self) {
        //immutable borrow
        if let Some(friend) = self.friends.first() {
            //mutable borrow
            self.heal(friend.loyalty); // compiler cannot see that heal only modified hp & sp, but not friend.
            println!("Healing for {}", friend.loyalty);
        }
    }

    pub fn heal(&mut self, amount: u8) {
        self.hp += amount;
        self.sp -= amount;
    }
}

fn main() {}
