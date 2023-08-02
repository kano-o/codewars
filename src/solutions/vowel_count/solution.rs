fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;

    for x in string.chars() {
        match x {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels_count += 1,
            _ => ()
        };
    }

    vowels_count
}

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}