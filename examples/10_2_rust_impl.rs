struct User {
    username: String,
    email: String,
    age: i32,
}

impl User {
    // Associated function (no self)
    fn new(username: String, email: String, age: i32) -> User {
        User {
            username,
            email,
            age,
        }
    }
    // Method with &self (can read data)
    fn display(&self) {
        println!(
            "User: {}, Email: {}, Age {}",
            self.username, self.email, self.age
        );
    }

    // Method with &mut self (can modify data)
    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }

    // Another associated function
    fn default_user() -> User {
        User {
            username: String::from("Guest"),
            email: String::from("guest@example.com"),
            age: 0,
        }
    }
}

fn main() {
    let mut user1: User = User {
        username: String::from("Alice"),
        email: String::from("alice@gmail.com"),
        age: 29,
    };

    user1.display();
    user1.update_email(String::from("roudra@gmail.com"));
    user1.display();

    // Called with :: (not .)
    let user2 = User::new(String::from("Bob"), String::from("bob@gmail.com"), 25);
    user2.display();

    let user3: User = User::default_user();
    user3.display();
}
