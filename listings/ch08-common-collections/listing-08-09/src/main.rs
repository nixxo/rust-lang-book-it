fn main() {
    // ANCHOR: here
    enum CellaFoglioDiCalcolo {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        CellaFoglioDiCalcolo::Int(3),
        CellaFoglioDiCalcolo::Text(String::from("blu")),
        CellaFoglioDiCalcolo::Float(10.12),
    ];
    // ANCHOR_END: here
}
