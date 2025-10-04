// ANCHOR: all
use blog::Post;

// ANCHOR: here
fn main() {
    let mut post = Post::new();

    post.aggiungi_testo("Oggi a pranzo ho mangiato un'instalata");
    assert_eq!("", post.contenuto());
    // ANCHOR_END: here

    post.richiedi_revisione();
    assert_eq!("", post.contenuto());

    post.approva();
    assert_eq!("Oggi a pranzo ho mangiato un'instalata", post.contenuto());
    // ANCHOR: here
}
// ANCHOR_END: here
// ANCHOR_END: all
