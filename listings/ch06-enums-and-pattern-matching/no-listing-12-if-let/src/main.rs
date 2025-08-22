fn main() {
    // ANCHOR: here
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Il massimo è configurato per essere {max}");
    }
    // ANCHOR_END: here
}
