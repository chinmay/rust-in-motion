/*
    use of ? operaton.
    usecase: twitter like function that lets you write status only 200 char or less.
    Assume save_status fn that wrotes to a database and return result
*/

// ' = lifecycle operator that keeps value alive till end of program scope.
// re
fn save_status(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("status text is too long");
    }

    // call to other function, retrive result and retrieve values from the Result.
    let record = match save_to_database(text) {
        Ok(rec) => rec,
        Err(e) => return Err(e),
    };

    // another way of doing same is: call function and follow it by ?
    // ex: let record = save_to_database(text)?;
    // if reading old code, previously it was implemented as try!
    // ex: let record = try!(save_to_database(text));

    Ok(record.id)
}

fn save_to_database(text: &str) -> Result<StatusRecord, &'static str> {
    //fake implement that always fails
    Err("database unavailable")
}

struct StatusRecord {
    id: i64,
    text: String,
    created_at: std::time::Instant,
}
