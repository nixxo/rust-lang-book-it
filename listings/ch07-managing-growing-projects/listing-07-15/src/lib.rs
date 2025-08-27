// ANCHOR: here
use std::fmt;
use std::io;

fn funzione1() -> fmt::Result {
    // --snip--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}

fn funzione2() -> io::Result<()> {
    // --snip--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
