fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut cubes_vec = cubes.to_vec();
    cubes_vec.sort();
    if dir == 'R' { return cubes_vec }
    else if dir == 'L' {
        cubes_vec.reverse();
        return cubes_vec
    } else {
        panic!("wrong char entered! expected L or R, found {}", dir)
    }
}



#[cfg(test)]
mod tests {
    use super::flip;

    #[test]
    fn sample_tests() {
        assert_eq!(flip('R', &vec![3, 2, 1, 2]), vec![1, 2, 2, 3]);
        assert_eq!(flip('L', &vec![1, 4, 5, 3, 5]), vec![5, 5, 4, 3, 1]);
    }
}
