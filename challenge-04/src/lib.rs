pub fn check_file(file: &String) -> bool {
    let first_part: Vec<char> = file.split("-").collect::<Vec<&str>>()[0].chars().collect::<Vec<char>>();
    let second_part: Vec<char> = file.split("-").collect::<Vec<&str>>()[1].chars().collect::<Vec<char>>();

    let mut first_part_filtered: Vec<char> = Vec::new();

    for character in first_part.clone() {
        if first_part.iter().filter(|&actual_char| *actual_char == character).count() == 1 && !first_part_filtered.contains(&character) {
            first_part_filtered.push(character);
        }
    }

    first_part_filtered == second_part
}

pub fn check_files(files: Vec<String>) -> Vec<String> {

    let mut correct_files: Vec<String> = Vec::new();

    for file in files {
        if check_file(&file) {
            correct_files.push(file);
        }
    }

    correct_files
}
