enum DoorState {
    Opened,
    Closed,
}

enum DoorAction {
    Open,
    Close,
}

fn take_action(current_state: DoorState, action: DoorAction) {
    match (current_state, action) {
        (DoorState::Opened, DoorAction::Close) => {
            println!("Closing opened door");
        }
        (DoorState::Closed, DoorAction::Open) => {
            unimplemented!();
        }
        _ => unreachable!("code is not reachable"),
    }
}

fn main() {
    take_action(DoorState::Opened, DoorAction::Close);
    take_action(DoorState::Opened, DoorAction::Open);
    take_action(DoorState::Closed, DoorAction::Open)
}
/*

unreachable! == impossible to get to this spot.
unimplemented! == code not implemented yet.
assert!, assert_eq!, assert_ne! == panic if condition not true.

*/
