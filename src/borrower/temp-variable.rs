pub struct Player {
    score: i32,
}

impl Player {
    pub fn set_score(&mut self, new_score: i32) {
        self.score = new_score;
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn new() -> Self {
        Player { score: 0 }
    }
}

fn main() {
    let mut player1 = Player::new();
    player1.set_score(player1.score() + 1);
    /*
    mutable & immutable borrowing happening in same line
    player1.score = immutable borro
    player.set = mutable borrow
    */

    //introducing temp varialbe.
    let old_score = player1.score();
    player1.set_score(old_score + 1);
}
