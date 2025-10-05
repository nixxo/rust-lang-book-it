fn ritorna_chiusura(init: i32) -> impl Fn(i32) -> i32 {
    move |x| x + init
}
