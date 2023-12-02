use challenge_04::check_file;

#[test]
fn test_basic() {
    assert_eq!(check_file(&String::from("xyzz33-xy")), true);

    assert_eq!(check_file(&String::from("abcca1-ab1")), false);

    assert_eq!(check_file(&String::from("abbc11-ca")), false);
}
