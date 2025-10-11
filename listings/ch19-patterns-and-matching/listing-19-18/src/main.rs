fn main() {
    // ANCHOR: here
    let mut valore_setting = Some(5);
    let nuovo_valore_setting = Some(10);

    match (valore_setting, nuovo_valore_setting) {
        (Some(_), Some(_)) => {
            println!("Non è possibile sovrascrivere un valore personalizzato esistente");
        }
        _ => {
            valore_setting = nuovo_valore_setting;
        }
    }

    println!("Valore setting è {valore_setting:?}");
    // ANCHOR_END: here
}
