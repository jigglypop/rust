
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user = User {
        email: String::from("someone@email.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };
    println!("{:#?}", user.email);
    println!("{:#?}", user.username);
    println!("{:#?}", user.sign_in_count);
    println!("{:#?}", user.active);
    user.email = String::from("new@email");
    println!("{:#?}", user.email);
    println!("{:#?}", user.username);
    println!("{:#?}", user.sign_in_count);
    println!("{:#?}", user.active);
}
