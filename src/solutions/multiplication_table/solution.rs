fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut table = vec![vec![0;len];len];
    for y in 0..len {
        for x in 0.. len {
            table[y][x] = (x + 1) * (y + 1);
        }
    }

    return table
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1,2,3], [2,4,6], [3,6,9]]);
    }
}