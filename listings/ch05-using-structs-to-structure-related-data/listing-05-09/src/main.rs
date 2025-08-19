fn main() {
    let rettangolo1 = (30, 50);

    println!(
        "L'area del rettangolo è di {} pixel quadrati.",
        area(rettangolo1)
    );
}

fn area(dimensioni: (u32, u32)) -> u32 {
    dimensioni.0 * dimensioni.1
}
