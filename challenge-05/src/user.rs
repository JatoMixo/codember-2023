pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub age: Option<u8>,
    pub location: Option<String>,
}

fn is_alphanumeric_string(string: &String) -> bool {
    for character in string.chars().into_iter() {
        if !character.is_alphanumeric() {
            return false;
        }
    }

    true
}

impl User {
    pub fn is_valid_id(&self) -> bool {
        is_alphanumeric_string(&self.id)
    }    

    pub fn is_valid_username(&self) -> bool {
        is_alphanumeric_string(&self.username)
    }

    fn check_email_domain(domain: &str) -> bool {

        const DOMAIN_ENDING_LENGTH: usize = 4; // ".com" Length
        if !domain.ends_with(".com") || domain.len() == DOMAIN_ENDING_LENGTH {
            return false;
        }

        true
    }

    pub fn is_valid_email(&self) -> bool {

        let email_parsed = self.email.split("@").collect::<Vec<&str>>();
        if email_parsed.len() != 2 {
            return false;
        }

        let user = email_parsed[0];
        if user.len() == 0 {
            return false;
        }

        let domain = email_parsed[1];
        if !User::check_email_domain(domain) {
            return false;
        }

        true
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_id() && self.is_valid_username() && self.is_valid_email()
    }
}
