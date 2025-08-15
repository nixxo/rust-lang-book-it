fn main() {
    let s = String::from("hello"); // `s` entra nello scope

    prende_ownership(s);           // il valore di `s` viene spostato nella funzione...
                                   // ... e quindi qui smette di esser valido

    let x = 5;                     // `x` entra nello scope

    duplica(x);                    // Siccome i32 implementa il tratto Copy,
                                   // `x` NON viene spostato nella funzione,
                                   // quindi dopo può ancora essere usata.

}   // Qui, `x` esce dallo scope, ed anche `s`. Tuttavia, siccome il valore di `s`
    // era stato spostato, non succede nulla di particolare.

fn prende_ownership(una_stringa: String) { // `una_stringa` entra nello scope
    println!("{una_stringa}");
}   // Qui, `una_stringa` esce dallo scope e `drop` viene chiamato.
    // La memoria corrispondente viene rilasciata.

fn duplica(un_integer: i32) { // `un_integer` entra nello scope
    println!("{un_integer}");
}   // Qui, `un_integer` esce dallo scope. Non succede nulla di particolare.
