fn main() {
    // ANCHOR: here
    let mut val_impostazioni = Some(5);
    let nuovo_val_impostazioni = Some(10);

    match (val_impostazioni, nuovo_val_impostazioni) {
        (Some(_), Some(_)) => {
            println!("Non è possibile sovrascrivere un valore personalizzato esistente");
        }
        _ => {
            val_impostazioni = nuovo_val_impostazioni;
        }
    }

    println!("Valore impostazioni è {val_impostazioni:?}");
    // ANCHOR_END: here
}
