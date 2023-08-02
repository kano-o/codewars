fn switch_it_up(n: usize) -> &'static str {
    ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(switch_it_up(1), "One");
        assert_eq!(switch_it_up(2), "Two");
        assert_eq!(switch_it_up(3), "Three");
    }
}