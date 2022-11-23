#[allow(clippy::needless_range_loop)]
/// Make a permutation for N qubits.
pub fn make_permutation(new_ordering: &[usize]) -> Vec<usize> {
    let n = new_ordering.len();
    let m = 1 << n;
    let mut x = vec![0; m];

    for i in 0..m {
        for j in 0..n {
            // TODO: Make this more readable... and try to understand it while you're at it.
            x[i] += (1 << (n - new_ordering[j] - 1)) * ((i >> (n - j - 1)) & 1);
        }
    }

    x
}
/// Make the inverse permutation for some defined permutation.
pub fn make_inverse_permutation(permutation: &[usize]) -> Vec<usize> {
    let n = permutation.len();
    let mut x = vec![0; n];

    for (i, &j) in permutation.iter().enumerate() {
        x[j] = i;
    }

    x
}

pub fn number_of_qubits(n: usize) -> usize {
    if n == 0 {
        // shouldn't be possible, but let's be thorough
        return 0;
    }

    let mut x = n;
    let mut r = 0;
    while x != 1 {
        x >>= 1;
        r += 1;
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(make_permutation(&[0, 1, 2]), vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(make_permutation(&[0, 2, 1]), vec![0, 2, 1, 3, 4, 6, 5, 7]);
        assert_eq!(make_permutation(&[1, 0, 2]), vec![0, 1, 4, 5, 2, 3, 6, 7]);
        assert_eq!(make_permutation(&[1, 2, 0]), vec![0, 4, 1, 5, 2, 6, 3, 7]);
        assert_eq!(make_permutation(&[2, 0, 1]), vec![0, 2, 4, 6, 1, 3, 5, 7]);
        assert_eq!(make_permutation(&[2, 1, 0]), vec![0, 4, 2, 6, 1, 5, 3, 7]);
    }
}
