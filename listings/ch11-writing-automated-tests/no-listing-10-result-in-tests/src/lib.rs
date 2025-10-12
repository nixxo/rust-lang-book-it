pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("due più due non fa quattro"))
        }
    }
}
// ANCHOR_END: here
