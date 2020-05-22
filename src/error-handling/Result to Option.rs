/*

whether value of Result is ok variant or error variant
Result ->   is_ok
            is_err

Option  ->  is_some
            is_none


Result -> Option
    use ok()
        ex: Ok(T).ok() -> Some(T)
            Err(E).ok() -> None

Option -> Result
    use ok_or()
        ex: Some(T).ok_or(err_value) -> Ok(T)
            None.ok_or(err_value) -> Err(err_value)

unwrap_or for fallback values
    Ok(T).unwrap_or(fallback) -> T
    Some(T).unwrap_or(fallback) -> T
    Err(E).unwrap_or(fallback) -> fallback
    None.unwrap_or(fallback) -> fallback



*/

fn main() {
    // This is anti-pattern
    let option_value = Some(25);
    if option_value.is_some() {
        let inner = option_value.unwrap();
        println!("inner={}", inner);
    }

    //better solution is
    if let Some(inner) = option_value {
        println!("inner={}", inner);
    }
}
