use edifier::string_helpers::*;

#[test]
fn simple_new_lines() {
    let mut test = "Aqui ((hay espacios) estes) o no estes".to_string();
    let actual = add_new_lines(test, 2);

    assert_eq!(actual, "Aqui (\n(hay espacios) estes) o no estes")
}

#[test]
fn onelevel_new_lines() {
    let mut test = "(Aqui ((())) ((())) estes o no estes)".to_string();
    let actual = add_new_lines(test, 1);

    assert_eq!(actual, "(Aqui\n((()))\n((())) estes o no estes)")
}

#[test]
fn complex_new_lines() {
    let mut test = "(Aqui((() ))((())) estes o no estes)".to_string();
    let actual = add_new_lines(test, 2);

    assert_eq!(actual, "(Aqui\n(\n(() ))\n(\n(())) estes o no estes)")
}
