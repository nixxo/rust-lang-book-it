use blog::Post;

fn main() {
    let mut post = Post::new();

    post.aggiungi_testo("Oggi a pranzo ho mangiato un'instalata");
    assert_eq!("", post.contenuto());

    post.richiedi_revisione();
    assert_eq!("", post.contenuto());

    post.approva();
    assert_eq!("Oggi a pranzo ho mangiato un'instalata", post.contenuto());
}
