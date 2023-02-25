struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("matt"),
        email: String::from("matt@gmail.com"),
        sign_in_count: 2,
    };

    println!(
        "Email: {email}, active: {active}, username: {username}, sign_in_count={sign_in_count}",
        email = user1.email,
        active = user1.active,
        username = user1.username,
        sign_in_count = user1.sign_in_count
    );

    let user2 = User {
        active: false,
        sign_in_count: 0,
        ..user1 // note that this removes ownership and invalidates user1
    };

    println!(
        "Email: {email}, active: {active}, username: {username}, sign_in_count={sign_in_count}",
        email = user2.email,
        active = user2.active,
        username = user2.username,
        sign_in_count = user2.sign_in_count
    );
}
