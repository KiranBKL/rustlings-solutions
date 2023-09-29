// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

// arrays are fixed-sized and vectors are dynamic in size

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a:[i32; 4] = [10, 20, 30, 40]; // a plain array
    let v:Vec<i32>= a.to_vec();// TODO: declare your vector here with the macro for vectors
   // let v:Vec<i32>= vec![50,60,70,80];

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
