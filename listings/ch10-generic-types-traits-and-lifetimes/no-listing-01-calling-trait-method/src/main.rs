use aggregatore::{PostSocial, Sommario};

fn main() {
    let post = PostSocial {
        nomeutente: String::from("horse_ebooks"),
        contenuto: String::from(
            "ovviamente, come probabilmente già sapete, gente",
        ),
        risposta: false,
        repost: false,
    };

    println!("1 nuovo post: {}", post.riassunto());
}
