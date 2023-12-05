mod user;
use user::User;

mod csv_reader;
use csv_reader::get_users;

fn main() {
    let mut solution: String = "".to_string();

    for user in get_users() {
        if !user.is_valid() {
            solution += &user.username.chars().nth(0).unwrap().to_string();
        }
    }

    println!("{}", solution);
}
