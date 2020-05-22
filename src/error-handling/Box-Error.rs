use std::env;
use std::error::Error;

//use Box<Error> when you dont know what will be exact error at compile time. here env or parse can throw diff errors.
fn num_threads() -> Result<usize, Box<Error>> {
    let s = env::var("NUM_THREADS")?;
    let n: usize = s.parse()?;
    Ok(n + 1)
}

fn run_application() -> Result<(), Box<Error>> {
    let num = num_threads()?;
    println!("number of threads is: {}", num);
    Ok(())
}

fn main() {
    if let Err(e) = run_application() {
        panic!("error happened {}", e)
    };
}
