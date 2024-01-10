
//-----------------------------------------------------------------
// Definition and Initialzation of Structs
//-----------------------------------------------------------------
struct User {
    active: bool,           // pair of variable : type
    username: String,
    email: String,
    sign_in_count: u64,     // comma separated and last comma is allowed
}                           // no semicolon

fn main() {
    //-----------------------------------------------------------------
    // Initialization of Structs
    //-----------------------------------------------------------------
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,


    }; // comma

    //println!("{user1.active} {user1.username} {user1.email} {sign_in_count}"); // NG: not supported formatted string
    println!("{} {} {} {}",
        user1.active, user1.username, user1.email, user1.sign_in_count);
}
