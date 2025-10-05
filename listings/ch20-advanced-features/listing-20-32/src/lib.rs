fn ritorna_chiusura() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
