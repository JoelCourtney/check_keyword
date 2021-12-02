use check_keyword::CheckKeyword;

#[test]
fn interface() {
    assert_eq!("match".into_safe(), "r#match");
}