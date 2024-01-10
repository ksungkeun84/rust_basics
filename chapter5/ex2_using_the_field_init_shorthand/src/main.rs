
//-----------------------------------------------------------------
// Using the Field Init Shorthand
//-----------------------------------------------------------------
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,              // If a field name is equivalent to a parameter name to assign, init can be shorthanded.
        username,
        sign_in_count: 1
    }
}

fn main() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

    println!("{} {} {} {}",
        user1.active,
        user1.username,
        user1.email,
        user1.sign_in_count,
    );
}
