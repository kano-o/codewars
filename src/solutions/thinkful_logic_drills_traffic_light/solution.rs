fn update_light(current: &str) -> String {
     match current {
        "green" => "yellow".to_string(),
        "yellow" => "red".to_string(),
        "red" => "green".to_string(),
        _ => "no valid colour found".to_string()
    }
}

#[test]
fn basic_test() {
    assert_eq!(update_light("green"), "yellow");
    assert_eq!(update_light("yellow"), "red");
    assert_eq!(update_light("red"), "green");
}