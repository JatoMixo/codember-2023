use crate::User;

pub fn get_users() -> Vec<User> {
    let mut users = Vec::new();

    let file = std::fs::read_to_string("users.csv").expect("Forgot the file :/");

    let mut csv_file = csv::Reader::from_reader(file.as_bytes());
    for user in csv_file.records() {
        let user = user.expect("Goddamn CSV is corrupted");

        let user_age: Option<String> = if &user[3] == "" {
            None
        } else {
            Some(user[3].to_string())
        };

        let user_location = if user[4].to_string() == "" {
            None
        } else {
            Some(user[4].to_string())
        };

        users.push(User {
            id: user[0].to_string(),
            username: user[1].to_string(),
            email: user[2].to_string(),
            age: user_age,
            location: user_location,
        });
    }

    users
}