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

    let user3 = User {
        username: String::from("eats_indigo"),
        email: String::from("eats_indigo@gmail.com"),
        ..user2 // note this does not invalidate user2 because we have assigned all heap allocated fields.
    };

    println!(
        "Email: {email}, active: {active}, username: {username}, sign_in_count={sign_in_count}",
        email = user2.email,
        active = user2.active,
        username = user2.username,
        sign_in_count = user2.sign_in_count
    );

    println!(
        "Email: {email}, active: {active}, username: {username}, sign_in_count={sign_in_count}",
        email = user3.email,
        active = user3.active,
        username = user3.username,
        sign_in_count = user3.sign_in_count
    );
}
