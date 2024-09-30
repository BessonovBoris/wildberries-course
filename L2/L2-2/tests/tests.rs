use L2_2::unpacking;

#[test]
fn test_regular() {
    let input = "abcd".to_string();
    let expect = "abcd".to_string();
    let actual = unpacking(&input).unwrap();

    assert_eq!(expect, actual);

    let input = "a3b1c".to_string();
    let expect = "aaabc".to_string();
    let actual = unpacking(&input).unwrap();

    assert_eq!(expect, actual);

    let input = "".to_string();
    let expect = "".to_string();
    let actual = unpacking(&input).unwrap();

    assert_eq!(expect, actual);
}

#[test]
fn test_escape() {
    let input = "qwe\\4\\5".to_string();
    let expect = "qwe45".to_string();
    let actual = unpacking(&input).unwrap();

    assert_eq!(expect, actual);

    let input = "a\\1\\23".to_string();
    let expect = "a1222".to_string();
    let actual = unpacking(&input).unwrap();

    assert_eq!(expect, actual);

    let input = "a\\\\5".to_string();
    let expect = "a\\\\\\\\\\".to_string();
    let actual = unpacking(&input).unwrap();

    assert_eq!(expect, actual);
}
