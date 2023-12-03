mod lib;
use lib::check_files;

fn main() {
    let files = std::fs::read_to_string("files.txt").expect("Forgot the file...");

    let mut files_vec = files.split("\n").collect::<Vec<&str>>();
    files_vec.remove(files_vec.len() - 1);

    println!("{:?}", check_files(files_vec)[32].split("-").collect::<Vec<&str>>()[1]);
}
