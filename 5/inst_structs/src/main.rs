fn main() {
    // Structs are like tuples but can hold different kind of data and explicitly need to be told what and where they will be holding that value. Structs can also be seen as a template where we have to fill out the data but we are given the fields with instructions what kind of data to fill in.
    {
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        // Here we have used struct with the properties that we defined above, we do not have to define them in the same order because it is a key value pair.
        let user1: User = User {
            active: true,
            username: String::from("someuser"),
            email: String::from("some@email.com"),
            sign_in_count: 1,
        };
        // To get a specific item out of the instance we use dot notation to refer to that key
        let user_email: &str = &user1.email;
        println!("User's Email is: {}", user_email);

        // We can also create mutable structs instances
        let mut user2: User = User {
            active: false,
            username: String::from("Someusere1"),
            email: String::from("email@email.com"),
            sign_in_count: 50,
        };

        user2.email = String::from("new@email.com");
        println!("User 2 email after change: {}", user_email);

        // This is a one way of createing a function that return the User struct withe values and we can set the parameter same as the keys, but we will see more simple way to do that without getting annoyed by typing those thing twice.
        fn build_user(email: String, username: String) -> User {
            User {
                active: true,
                username: username,
                email: email,
                sign_in_count: 1,
            }
        }

        // Here we have named the parameters same as the key, and since they are the same we do not have to pass them again to that particular key as a value.
        fn build_user2(email: String, username: String) -> User {
            User {
                active: true,
                username,
                email,
                sign_in_count: 1,
            }
        }
    }

    {
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        let user1 = User {
            active: true,
            username: String::from("someuser"),
            email: String::from("some@email.com"),
            sign_in_count: 1,
        };

        // Here instead of adding values we are just copying the values that exist in the user1 but there is more simpler way to do this.
        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("some@email.com"),
            sign_in_count: user1.sign_in_count,
        };

        // Here we have successfully copied the data from user2 with email being unique. We can choose to update manually how many fields we want, after being done with that and using .. syntax it will automatically gather the info for the fields that have not been updated.
        let user3: User = User {
            email: String::from("email@email.com"),
            ..user2
        };
    }

    // Creating different types with tuple structs
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black: Color = Color(0, 0, 0);
        let origin: Point = Point(0, 0, 0);
    }
}
