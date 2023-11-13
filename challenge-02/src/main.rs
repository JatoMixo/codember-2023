fn main() {
    const ORDER: &str = "&###@&*&###@@##@##&######@@#####@#@#@#@##@@@@@@@@@@@@@@@*&&@@@@@@@@@####@@@@@@@@@#########&#&##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@##@@&";

    let mut number = 0;

    for operation in ORDER.chars() {
        match operation {
            '#' => number += 1,
            '@' => number -= 1,
            '*' => number *= number,
            '&' => print!("{}", number),
            _ => panic!("You goddamn inserted a wrong char"),
        }
    }
}
