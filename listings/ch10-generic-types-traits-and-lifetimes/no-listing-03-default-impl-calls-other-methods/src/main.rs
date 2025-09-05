use aggregatore::{self, SocialPost, Sommario};

fn main() {
    // ANCHOR: here
    let post = SocialPost {
        nomeutente: String::from("horse_ebooks"),
        contenuto: String::from(
            "ovviamente, come probabilmente già sapete, gente",
        ),
        risposta: false,
        repost: false,
    };

    println!("1 nuovo post: {}", post.riassunto());
    // ANCHOR_END: here
}
