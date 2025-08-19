struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "qualcuno@mia_mail.com",
        username: "qualcuno123",
        active: true,
        sign_in_count: 1,
    };
}
