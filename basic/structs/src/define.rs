struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn init_user() -> User {
    User {
	email: String::from("abc@abc.com"),
	username: "wanghao".to_string(),
	active: true,
	sign_in_count: 1,
    }
}

fn init_user2(email: String, username: String) -> User {
    User {
	email,
	username,
	active: true,
	sign_in_count: 1,
    }
}
