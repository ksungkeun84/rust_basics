
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
fn main() {
    let user1 = build_user(String::from("sk"), String::from("sk@example.com"));

    let user2 = User {
        username: String::from("sk84"), // order doesn't matter
    ..user1 // it should be places last
    };

    // NG: user1 is dropped because ownership of username and email of user1 are moved to user2
    //println!("{} {} {} {}",
    //    user1.active,
    //    user1.username,
    //    user1.email,
    //    user1.sign_in_count,
    //);
    println!("{} {} {} {}",
        user2.active,
        user2.username,
        user2.email,
        user2.sign_in_count,
    );
}
