fn increment_string(s: &str) -> String {
    todo!();
    let mut string = s.to_string();
    s.chars().enumerate().for_each(|(i, c)| { });
    for (i, c) in string.chars().enumerate() {
        println!("{}", i);
        if i == s.chars().count() - 1 {
            if c.is_numeric() { string.insert(i, '1'); }
            else if c.is_alphabetic() { string.insert(i + 1, '1'); }
        }
    }
    println!("{}", string);
    String::from(s)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected,
                "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
    }
}
