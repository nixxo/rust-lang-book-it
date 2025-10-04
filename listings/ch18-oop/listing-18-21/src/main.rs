use blog::Post;

fn main() {
    let mut post = Post::new();

    post.aggiungi_testo("Oggi a pranzo ho mangiato un'instalata");

    let post = post.richiedi_revisione();

    let post = post.approva();

    assert_eq!("Oggi a pranzo ho mangiato un'instalata", post.contenuto());
}
