// struct
struct User {
    active: bool,
    username: String,
    email: String,
    // we use String instead of &str, because we want each struct to own its own data
    // to use &str, we'd have to use lifetimes (that ensures the data referenced is valid for as long as the struct is)
    sign_in_count: u64,
}

// tuple structs (without names)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// unit-like structs (no fields)
struct AlwaysEqual;

fn main() {
    // immutable user
    let _user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // mutable user
    let mut _user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    _user2.email = String::from("user@mail.com");

    // using a builder function
    let _user3 = build_user(
        String::from("user@mail.com"),
        String::from("user")
    );

    // copying another user
    let _user4 = User {
        email: String::from("user1@mail.com"),
        .._user1
    };

    // using a tuple struct
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // using a unit-like struct
    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand for email: email
        username, // shorthand for username: username
        active: true,
        sign_in_count: 1,
    }
}