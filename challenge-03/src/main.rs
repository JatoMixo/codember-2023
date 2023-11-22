struct EncryptionRule {
    min: u8,
    max: u8,
    character: char,
}

impl EncryptionRule {
    fn from_string(mut string: String) -> EncryptionRule {

        string = string.replace(" ", "-");
        let values: Vec<&str> = string.split('-').collect();

        EncryptionRule {
            min: values[0].parse::<u8>().unwrap(),
            max: values[1].parse::<u8>().unwrap(),
            character: values[2].chars().nth(0).unwrap(),
        }
    }
}

fn main() {
    let full_rules: &str = &std::fs::read_to_string("strings.txt").expect("Forgot the file...");

    let rules: Vec<&str> = full_rules.split('\n').collect();
    let mut invalid_strings: Vec<&str> = Vec::new();

    rules.into_iter().for_each(|rule| {
        let encryption_rule: EncryptionRule = EncryptionRule::from_string(rule.to_string());
        let key = rule.split(": ").collect::<Vec<&str>>()[1];

        let char_count = key.chars().filter(|character| character == &encryption_rule.character).count() as u8;
        if char_count > encryption_rule.max || char_count < encryption_rule.min {
            invalid_strings.push(rule);
        }
    });

    println!("{}", invalid_strings[41]);
}
