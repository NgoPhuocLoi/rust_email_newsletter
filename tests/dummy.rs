use claim::assert_ok;

#[test]
fn dummy_result_test() {
    let r: Result<&str, &str> = Err("Error happened");
    assert_ok!(r);
}
