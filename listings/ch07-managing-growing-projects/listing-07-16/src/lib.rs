// ANCHOR: here
use std::fmt::Result;
use std::io::Result as IoResult;

fn funzione1() -> Result {
    // --taglio--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}

fn funzione2() -> IoResult<()> {
    // --taglio--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
